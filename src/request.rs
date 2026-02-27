// Modified version of the file openapi-generator would usually put in
// apis/request.rs

use std::{collections::HashMap, time::Duration};

use http::header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, HeaderValue, USER_AGENT};
use http_body_util::{BodyExt as _, Full};
use hyper::body::Bytes;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use rand::Rng;
use serde::de::DeserializeOwned;

use crate::{Configuration, error::Error};

const APPLICATION_MSGPACK: HeaderValue = HeaderValue::from_static("application/msgpack");

#[allow(dead_code)]
pub(crate) enum Auth {
    None,
    Bearer,
}

/// If the authorization type is unspecified then it will be automatically
/// detected based on the configuration. This functionality is useful when the
/// OpenAPI definition does not include an authorization scheme.
#[derive(Clone)]
pub(crate) struct Request {
    method: http::Method,
    path: &'static str,
    query_params: HashMap<&'static str, String>,
    no_return_type: bool,
    path_params: HashMap<&'static str, String>,
    header_params: HashMap<&'static str, String>,
    serialized_body: Option<Vec<u8>>,
}

impl Request {
    pub(crate) fn new(method: http::Method, path: &'static str) -> Self {
        Request {
            method,
            path,
            query_params: HashMap::new(),
            path_params: HashMap::new(),
            header_params: HashMap::new(),
            serialized_body: None,
            no_return_type: false,
        }
    }

    pub(crate) fn with_body<T: serde::Serialize>(mut self, param: T) -> Self {
        self.serialized_body = Some(rmp_serde::to_vec_named(&param).unwrap());
        self
    }

    pub(crate) async fn execute<T: DeserializeOwned>(
        self,
        conf: &Configuration,
    ) -> Result<T, Error> {
        match self.execute_with_backoff(conf).await? {
            // This is a hack; if there's no_ret_type, T is (), but rmp_serde gives an
            // error when deserializing b"" into (), so deserialize a msgpack null into
            // it instead.
            // An alternate option would be to require T: Default, and then return
            // T::default() here instead since () implements that, but then we'd
            // need to impl default for all models.
            None => Ok(rmp_serde::from_slice(&[0xc0]).expect("serde null value")),
            Some(bytes) => Ok(rmp_serde::from_slice(&bytes).map_err(Error::generic)?),
        }
    }

    async fn execute_with_backoff(self, conf: &Configuration) -> Result<Option<Bytes>, Error> {
        let no_return_type = self.no_return_type;

        const MAX_BACKOFF: Duration = Duration::from_secs(5);

        let retry_schedule = match &conf.retry_schedule {
            Some(schedule) => schedule,
            None => &std::iter::successors(Some(Duration::from_millis(20)), |last_backoff| {
                Some(MAX_BACKOFF.min(*last_backoff * 2))
            })
            .take(conf.num_retries as usize)
            .collect(),
        };
        let mut retries = retry_schedule.iter();

        let mut request = self.build_request(conf)?;
        request
            .headers_mut()
            .insert("svix-req-id", rand::rng().random::<u32>().into());

        let mut retry_count = 0;

        let execute_request = async |request| {
            let response = conf.client.request(request).await.map_err(Error::generic)?;

            let status = response.status();
            if !status.is_success() {
                Err(Error::from_response(status, response.into_body()).await)
            } else if no_return_type {
                Ok(None)
            } else {
                let bytes = response
                    .into_body()
                    .collect()
                    .await
                    .map_err(Error::generic)?
                    .to_bytes();
                Ok(Some(bytes))
            }
        };

        loop {
            let request_fut = execute_request(request.clone());
            let res = if let Some(duration) = conf.timeout {
                tokio::time::timeout(duration, request_fut)
                    .await
                    .map_err(Error::generic)?
            } else {
                request_fut.await
            };

            let next_backoff = retries.next().copied();

            match res {
                Ok(result) => return Ok(result),
                e @ Err(Error::Validation(_)) => return e,
                Err(Error::Http(err)) if err.status.as_u16() < 500 => return Err(Error::Http(err)),
                e @ Err(_) => {
                    if next_backoff.is_none() {
                        return e;
                    }
                }
            }

            tokio::time::sleep(next_backoff.expect("next_backoff is always Some")).await;
            retry_count += 1;

            request
                .headers_mut()
                .insert("svix-retry-count", retry_count.into());
        }
    }

    fn build_request(self, conf: &Configuration) -> Result<http::Request<Full<Bytes>>, Error> {
        const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
        const PATH: &AsciiSet = &FRAGMENT.add(b'#').add(b'?').add(b'{').add(b'}');
        const PATH_SEGMENT: &AsciiSet = &PATH.add(b'/').add(b'%');

        let mut path = self.path.to_owned();
        for (k, v) in self.path_params {
            // replace {id} with the value of the id path param
            let percent_encoded_path_param_value =
                utf8_percent_encode(&v, PATH_SEGMENT).to_string();
            path = path.replace(&format!("{{{k}}}"), &percent_encoded_path_param_value);
        }

        let mut uri = format!("{}{}", conf.base_path, path);

        let mut query_string = form_urlencoded::Serializer::new("".to_owned());
        for (key, val) in self.query_params {
            query_string.append_pair(key, &val);
        }

        let query_string_str = query_string.finish();
        if !query_string_str.is_empty() {
            uri += "?";
            uri += &query_string_str;
        }

        let uri = http::Uri::try_from(uri).map_err(Error::generic)?;
        let mut req_builder = http::Request::builder().uri(uri).method(self.method);

        let mut request = if let Some(body) = self.serialized_body {
            let req_headers = req_builder.headers_mut().unwrap();
            req_headers.insert(CONTENT_TYPE, APPLICATION_MSGPACK);
            req_headers.insert(CONTENT_LENGTH, body.len().into());
            req_builder.body(Full::from(body)).map_err(Error::generic)?
        } else {
            req_builder.body(Full::default()).map_err(Error::generic)?
        };

        let request_headers = request.headers_mut();
        request_headers.insert(ACCEPT, APPLICATION_MSGPACK);

        // Detect the authorization type if it hasn't been set.
        let auth = if conf.bearer_access_token.is_some() {
            Auth::Bearer
        } else {
            Auth::None
        };
        match auth {
            Auth::Bearer => {
                if let Some(token) = &conf.bearer_access_token {
                    let value = format!("Bearer {token}")
                        .try_into()
                        .map_err(Error::generic)?;
                    request_headers.insert(AUTHORIZATION, value);
                }
            }
            Auth::None => {}
        }

        if let Some(user_agent) = &conf.user_agent {
            let value = user_agent.try_into().map_err(Error::generic)?;
            request_headers.insert(USER_AGENT, value);
        }

        for (k, v) in self.header_params {
            let v = v.try_into().map_err(Error::generic)?;
            request_headers.insert(k, v);
        }

        Ok(request)
    }
}
