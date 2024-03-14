// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use time::OffsetDateTime;

#[derive(thiserror::Error, Debug)]
pub enum WebhookError {
    #[error("failed to parse timestamp")]
    InvalidTimestamp,

    #[error("invalid secret")]
    InvalidSecret(#[from] base64::DecodeError),

    #[error("invalid header {0}")]
    InvalidHeader(&'static str),

    #[error("signature timestamp too old")]
    TimestampTooOldError,

    #[error("signature timestamp too far in future")]
    FutureTimestampError,

    #[error("missing header {0}")]
    MissingHeader(&'static str),

    #[error("signature invalid")]
    InvalidSignature,

    #[error("payload invalid")]
    InvalidPayload,
}

pub struct Webhook {
    key: Vec<u8>,
}

const PREFIX: &str = "whsec_";
const SVIX_MSG_ID_KEY: &str = "svix-id";
const SVIX_MSG_SIGNATURE_KEY: &str = "svix-signature";
const SVIX_MSG_TIMESTAMP_KEY: &str = "svix-timestamp";
const UNBRANDED_MSG_ID_KEY: &str = "webhook-id";
const UNBRANDED_MSG_SIGNATURE_KEY: &str = "webhook-signature";
const UNBRANDED_MSG_TIMESTAMP_KEY: &str = "webhook-timestamp";
const TOLERANCE_IN_SECONDS: i64 = 5 * 60;
const SIGNATURE_VERSION: &str = "v1";

impl Webhook {
    pub fn new(secret: &str) -> Result<Self, WebhookError> {
        let secret = secret.strip_prefix(PREFIX).unwrap_or(secret);
        let key = base64::decode(secret)?;

        Ok(Webhook { key })
    }

    pub fn from_bytes(secret: Vec<u8>) -> Result<Self, WebhookError> {
        Ok(Webhook { key: secret })
    }

    pub fn verify<HM: HeaderMap>(&self, payload: &[u8], headers: &HM) -> Result<(), WebhookError> {
        let msg_id = Self::get_header(headers, SVIX_MSG_ID_KEY, UNBRANDED_MSG_ID_KEY, "id")?;
        let msg_signature = Self::get_header(
            headers,
            SVIX_MSG_SIGNATURE_KEY,
            UNBRANDED_MSG_SIGNATURE_KEY,
            "signature",
        )?;
        let msg_ts = Self::get_header(
            headers,
            SVIX_MSG_TIMESTAMP_KEY,
            UNBRANDED_MSG_TIMESTAMP_KEY,
            "timestamp",
        )
        .and_then(Self::parse_timestamp)?;

        Self::verify_timestamp(msg_ts)?;

        let versioned_signature = self.sign(msg_id, msg_ts, payload)?;
        let expected_signature = versioned_signature
            .split_once(',')
            .map(|x| x.1)
            .ok_or(WebhookError::InvalidSignature)?;

        msg_signature
            .split(' ')
            .filter_map(|x| x.split_once(','))
            .filter(|x| x.0 == SIGNATURE_VERSION)
            .any(|x| {
                (x.1.len() == expected_signature.len())
                    && (x
                        .1
                        .bytes()
                        .zip(expected_signature.bytes())
                        .fold(0, |acc, (a, b)| acc | (a ^ b))
                        == 0)
            })
            .then_some(())
            .ok_or(WebhookError::InvalidSignature)
    }

    pub fn sign(
        &self,
        msg_id: &str,
        timestamp: i64,
        payload: &[u8],
    ) -> Result<String, WebhookError> {
        let payload = std::str::from_utf8(payload).map_err(|_| WebhookError::InvalidPayload)?;
        let to_sign = format!("{msg_id}.{timestamp}.{payload}",);
        let signed = hmac_sha256::HMAC::mac(to_sign.as_bytes(), &self.key);
        let encoded = base64::encode(signed);

        Ok(format!("{SIGNATURE_VERSION},{encoded}"))
    }

    fn get_header<'a, HM: HeaderMap>(
        headers: &'a HM,
        svix_hdr: &'static str,
        unbranded_hdr: &'static str,
        err_name: &'static str,
    ) -> Result<&'a str, WebhookError> {
        use private::HeaderValueSealed as _;

        headers
            ._get(svix_hdr)
            .or_else(|| headers._get(unbranded_hdr))
            .ok_or(WebhookError::MissingHeader(err_name))?
            ._to_str()
            .ok_or(WebhookError::InvalidHeader(err_name))
    }

    fn parse_timestamp(hdr: &str) -> Result<i64, WebhookError> {
        str::parse::<i64>(hdr).map_err(|_| WebhookError::InvalidTimestamp)
    }

    fn verify_timestamp(ts: i64) -> Result<(), WebhookError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        if now - ts > TOLERANCE_IN_SECONDS {
            Err(WebhookError::TimestampTooOldError)
        } else if ts > now + TOLERANCE_IN_SECONDS {
            Err(WebhookError::FutureTimestampError)
        } else {
            Ok(())
        }
    }
}

/// Trait to abstract over the `HeaderMap` types from both v0.2 and v1.0 of the
/// `http` crate.
pub trait HeaderMap: private::HeaderMapSealed {}

impl HeaderMap for http02::HeaderMap {}
impl HeaderMap for http1::HeaderMap {}

mod private {
    pub trait HeaderMapSealed {
        type HeaderValue: HeaderValueSealed;
        fn _get(&self, name: &str) -> Option<&Self::HeaderValue>;
    }

    impl HeaderMapSealed for http02::HeaderMap {
        type HeaderValue = http02::HeaderValue;
        fn _get(&self, name: &str) -> Option<&Self::HeaderValue> {
            self.get(name)
        }
    }
    impl HeaderMapSealed for http1::HeaderMap {
        type HeaderValue = http1::HeaderValue;
        fn _get(&self, name: &str) -> Option<&Self::HeaderValue> {
            self.get(name)
        }
    }

    pub trait HeaderValueSealed {
        fn _to_str(&self) -> Option<&str>;
    }

    impl HeaderValueSealed for http02::HeaderValue {
        fn _to_str(&self) -> Option<&str> {
            self.to_str().ok()
        }
    }
    impl HeaderValueSealed for http1::HeaderValue {
        fn _to_str(&self) -> Option<&str> {
            self.to_str().ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use http02::HeaderMap;
    use time::OffsetDateTime;

    use super::{
        Webhook, SVIX_MSG_ID_KEY, SVIX_MSG_SIGNATURE_KEY, SVIX_MSG_TIMESTAMP_KEY,
        UNBRANDED_MSG_ID_KEY, UNBRANDED_MSG_SIGNATURE_KEY, UNBRANDED_MSG_TIMESTAMP_KEY,
    };

    fn get_svix_headers(msg_id: &str, signature: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(SVIX_MSG_ID_KEY, msg_id.parse().unwrap());
        headers.insert(SVIX_MSG_SIGNATURE_KEY, signature.parse().unwrap());
        headers.insert(
            SVIX_MSG_TIMESTAMP_KEY,
            OffsetDateTime::now_utc()
                .unix_timestamp()
                .to_string()
                .parse()
                .unwrap(),
        );
        headers
    }

    fn get_unbranded_headers(msg_id: &str, signature: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(UNBRANDED_MSG_ID_KEY, msg_id.parse().unwrap());
        headers.insert(UNBRANDED_MSG_SIGNATURE_KEY, signature.parse().unwrap());
        headers.insert(
            UNBRANDED_MSG_TIMESTAMP_KEY,
            OffsetDateTime::now_utc()
                .unix_timestamp()
                .to_string()
                .parse()
                .unwrap(),
        );
        headers
    }

    #[test]
    fn test_sign() {
        let wh = Webhook::new("whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD").unwrap();
        assert_eq!(
            "v1,tZ1I4/hDygAJgO5TYxiSd6Sd0kDW6hPenDe+bTa3Kkw=".to_owned(),
            wh.sign(
                "msg_27UH4WbU6Z5A5EzD8u03UvzRbpk",
                1649367553,
                br#"{"email":"test@example.com","username":"test_user"}"#
            )
            .unwrap()
        );
    }

    #[test]
    fn test_verify() {
        let secret = "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD".to_owned();
        let msg_id = "msg_27UH4WbU6Z5A5EzD8u03UvzRbpk";
        let payload = br#"{"email":"test@example.com","username":"test_user"}"#;
        let wh = Webhook::new(&secret).unwrap();

        let signature = wh
            .sign(msg_id, OffsetDateTime::now_utc().unix_timestamp(), payload)
            .unwrap();
        for headers in [
            get_svix_headers(msg_id, &signature),
            get_unbranded_headers(msg_id, &signature),
        ] {
            wh.verify(payload, &headers).unwrap();
        }
    }

    #[test]
    fn test_no_verify() {
        let secret = "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD".to_owned();
        let msg_id = "msg_27UH4WbU6Z5A5EzD8u03UvzRbpk";
        let payload = br#"{"email":"test@example.com","username":"test_user"}"#;
        let wh = Webhook::new(&secret).unwrap();

        let signature = "v1,R3PTzyfHASBKHH98a7yexTwaJ4yNIcGhFQc1yuN+BPU=".to_owned();
        for headers in [
            get_svix_headers(msg_id, &signature),
            get_unbranded_headers(msg_id, &signature),
        ] {
            assert!(wh.verify(payload, &headers).is_err());
        }
    }

    #[test]
    fn test_verify_partial_signature() {
        let secret = "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD".to_owned();
        let msg_id = "msg_27UH4WbU6Z5A5EzD8u03UvzRbpk";
        let payload = br#"{"email":"test@example.com","username":"test_user"}"#;
        let wh = Webhook::new(&secret).unwrap();

        let signature = wh
            .sign(msg_id, OffsetDateTime::now_utc().unix_timestamp(), payload)
            .unwrap();

        // Just `v1,`
        for mut headers in [
            get_svix_headers(msg_id, &signature),
            get_unbranded_headers(msg_id, &signature),
        ] {
            let partial = format!(
                "{},",
                signature.split(',').collect::<Vec<&str>>().first().unwrap()
            );
            headers.insert(SVIX_MSG_SIGNATURE_KEY, partial.parse().unwrap());
            headers.insert(UNBRANDED_MSG_SIGNATURE_KEY, partial.parse().unwrap());
            assert!(wh.verify(payload, &headers).is_err());
        }

        // Non-empty but still partial signature (first few bytes)
        for mut headers in [
            get_svix_headers(msg_id, &signature),
            get_unbranded_headers(msg_id, &signature),
        ] {
            let partial = &signature[0..8];
            headers.insert(SVIX_MSG_SIGNATURE_KEY, partial.parse().unwrap());
            headers.insert(UNBRANDED_MSG_SIGNATURE_KEY, partial.parse().unwrap());
            assert!(wh.verify(payload, &headers).is_err());
        }
    }

    #[test]
    fn test_verify_incorrect_timestamp() {
        let secret = "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD".to_owned();
        let msg_id = "msg_27UH4WbU6Z5A5EzD8u03UvzRbpk";
        let payload = br#"{"email":"test@example.com","username":"test_user"}"#;
        let wh = Webhook::new(&secret).unwrap();

        let signature = wh
            .sign(msg_id, OffsetDateTime::now_utc().unix_timestamp(), payload)
            .unwrap();

        let mut headers = get_svix_headers(msg_id, &signature);
        for ts in [
            OffsetDateTime::now_utc().unix_timestamp() - (super::TOLERANCE_IN_SECONDS + 1),
            OffsetDateTime::now_utc().unix_timestamp() + (super::TOLERANCE_IN_SECONDS + 1),
        ] {
            headers.insert(
                super::SVIX_MSG_TIMESTAMP_KEY,
                ts.to_string().parse().unwrap(),
            );

            assert!(wh.verify(payload, &headers,).is_err());
        }
    }

    #[test]
    fn test_verify_with_multiple_signatures() {
        let secret = "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD".to_owned();
        let msg_id = "msg_27UH4WbU6Z5A5EzD8u03UvzRbpk";
        let payload = br#"{"email":"test@example.com","username":"test_user"}"#;
        let wh = Webhook::new(&secret).unwrap();

        let signature = wh
            .sign(msg_id, OffsetDateTime::now_utc().unix_timestamp(), payload)
            .unwrap();

        let multi_sig = format!(
            "{} {} {} {}",
            "v1,tFtCZ5RDCPxzWQRWXWPgrCgE2frDBe9gjpbWQxnVfsQ=",
            "v1,Mm7xgUVICxZfQ3bgf0h0Dof65L/IFx+PnZvnDWPCX6Q=",
            signature,
            "v1,9DfC1c3eeOrXB6w/5dIDydLNQaEyww5KalE5jLBZucE=",
        );

        let headers = get_svix_headers(msg_id, &multi_sig);

        wh.verify(payload, &headers).unwrap();
    }

    #[test]
    fn test_no_verify_with_multiple_signatures() {
        let secret = "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD".to_owned();
        let msg_id = "msg_27UH4WbU6Z5A5EzD8u03UvzRbpk";
        let payload = br#"{"email":"test@example.com","username":"test_user"}"#;
        let wh = Webhook::new(&secret).unwrap();

        let missing_sig = format!(
            "{} {} {}",
            "v1,tFtCZ5RDCPxzWQRWXWPgrCgE2frDBe9gjpbWQxnVfsQ=",
            "v1,Mm7xgUVICxZfQ3bgf0h0Dof65L/IFx+PnZvnDWPCX6Q=",
            "v1,9DfC1c3eeOrXB6w/5dIDydLNQaEyww5KalE5jLBZucE=",
        );

        let headers = get_svix_headers(msg_id, &missing_sig);

        assert!(wh.verify(payload, &headers).is_err());
    }

    #[test]
    fn test_missing_headers() {
        let secret = "whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD".to_owned();
        let msg_id = "msg_27UH4WbU6Z5A5EzD8u03UvzRbpk";
        let payload = br#"{"email":"test@example.com","username":"test_user"}"#;
        let wh = Webhook::new(&secret).unwrap();

        let signature = wh
            .sign(msg_id, OffsetDateTime::now_utc().unix_timestamp(), payload)
            .unwrap();
        for (mut hdr_map, hdrs) in [
            (
                get_svix_headers(msg_id, &signature),
                [
                    SVIX_MSG_ID_KEY,
                    SVIX_MSG_SIGNATURE_KEY,
                    SVIX_MSG_TIMESTAMP_KEY,
                ],
            ),
            (
                get_unbranded_headers(msg_id, &signature),
                [
                    UNBRANDED_MSG_ID_KEY,
                    UNBRANDED_MSG_SIGNATURE_KEY,
                    UNBRANDED_MSG_TIMESTAMP_KEY,
                ],
            ),
        ] {
            for hdr in hdrs {
                hdr_map.remove(hdr);
                assert!(wh.verify(payload, &hdr_map).is_err());
            }
        }
    }
}
