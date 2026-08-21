// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::fmt::Debug;

use base64::{Engine, engine::general_purpose::STANDARD};
use chacha20poly1305::{
    Key, XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit},
};
use ed25519_compact::*;

use crate::error::Result;

// Asymmetric Signature keys
#[derive(Clone, Eq)]
pub struct AsymmetricKey(pub KeyPair);

impl AsymmetricKey {
    pub fn generate() -> AsymmetricKey {
        AsymmetricKey(KeyPair::from_seed(Seed::generate()))
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        Ok(AsymmetricKey(KeyPair::from_slice(bytes).map_err(|_| {
            crate::error::Error::generic("Failed parsing key.")
        })?))
    }

    pub fn from_base64(b64: &str) -> Result<Self> {
        let bytes = STANDARD
            .decode(b64)
            .map_err(|_| crate::error::Error::generic("Failed parsing base64"))?;

        Self::from_slice(bytes.as_slice())
    }

    pub fn pubkey(&self) -> &[u8] {
        &self.0.pk[..]
    }
}

impl Debug for AsymmetricKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<AsymmetricKey sk=*** pk={}>",
            STANDARD.encode(self.0.pk.as_slice())
        )
    }
}

impl PartialEq for AsymmetricKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_slice() == other.0.as_slice()
    }
}

#[derive(Clone, Debug)]
pub struct Encryption(Option<Key>);

impl Encryption {
    const NONCE_SIZE: usize = 24;

    /// First byte of an encrypted-at-rest payload, distinguishing it from plaintext.
    /// Payloads are JSON or UTF-8 strings, so plaintext can never start with `0x00`.
    const PAYLOAD_MARKER: u8 = 0x00;

    pub fn new_noop() -> Self {
        Self(None)
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self(Some(Key::from_slice(&key).to_owned()))
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if let Some(main_key) = self.0.as_ref() {
            let cipher = XChaCha20Poly1305::new(main_key);
            let nonce: [u8; Self::NONCE_SIZE] = rand::random();
            let nonce = XNonce::from_slice(&nonce);
            let mut ciphertext = cipher
                .encrypt(nonce, data)
                .map_err(|_| crate::error::Error::generic("Encryption failed"))?;
            let mut ret = nonce.to_vec();
            ret.append(&mut ciphertext);
            Ok(ret)
        } else {
            Ok(data.to_vec())
        }
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if let Some(main_key) = self.0.as_ref() {
            let cipher = XChaCha20Poly1305::new(main_key);
            let nonce = &ciphertext[..Self::NONCE_SIZE];
            let ciphertext = &ciphertext[Self::NONCE_SIZE..];
            cipher
                .decrypt(XNonce::from_slice(nonce), ciphertext)
                .map_err(|_| crate::error::Error::generic("Encryption failed"))
        } else {
            Ok(ciphertext.to_vec())
        }
    }

    /// Encrypts a payload for storage with a [`Self::PAYLOAD_MARKER`] prefix.
    /// No-op when encryption is disabled.
    pub fn encrypt_payload(&self, data: &[u8]) -> Result<Vec<u8>> {
        if self.enabled() {
            let mut ret = vec![Self::PAYLOAD_MARKER];
            ret.append(&mut self.encrypt(data)?);
            Ok(ret)
        } else {
            Ok(data.to_vec())
        }
    }

    /// Decrypts a stored payload. Unmarked payloads predate encryption at rest
    /// and are returned as-is.
    pub fn decrypt_payload(&self, data: &[u8]) -> Result<Vec<u8>> {
        match data.split_first() {
            Some((&Self::PAYLOAD_MARKER, ciphertext)) => {
                if self.enabled() {
                    self.decrypt(ciphertext)
                } else {
                    Err(crate::error::Error::generic(
                        "main_secret unset, can't decrypt payload",
                    ))
                }
            }
            _ => Ok(data.to_vec()),
        }
    }

    pub fn enabled(&self) -> bool {
        self.0.is_some()
    }
}

impl Default for Encryption {
    fn default() -> Self {
        Self::new_noop()
    }
}

#[cfg(test)]
mod tests {
    use super::Encryption;

    #[test]
    fn test_encryption() {
        let clear = b"hello world";
        let encryption = Encryption::new([1; 32]);
        let ciphertext = encryption.encrypt(clear).unwrap();
        let clear2 = encryption.decrypt(&ciphertext).unwrap();
        assert_eq!(&clear[..], &clear2[..]);
    }

    #[test]
    fn test_payload_round_trip() {
        let clear = br#"{"hello":"world"}"#;
        let encryption = Encryption::new([1; 32]);
        let ciphertext = encryption.encrypt_payload(clear).unwrap();
        assert_ne!(&ciphertext[..], &clear[..]);
        assert!(!ciphertext.starts_with(b"{"));
        let clear2 = encryption.decrypt_payload(&ciphertext).unwrap();
        assert_eq!(&clear[..], &clear2[..]);
    }

    #[test]
    fn test_payload_noop_passthrough() {
        let clear = br#"{"hello":"world"}"#;
        let encryption = Encryption::new_noop();
        let ciphertext = encryption.encrypt_payload(clear).unwrap();
        assert_eq!(&ciphertext[..], &clear[..]);
        let clear2 = encryption.decrypt_payload(&ciphertext).unwrap();
        assert_eq!(&clear[..], &clear2[..]);
    }

    #[test]
    fn test_payload_plaintext_backward_compat() {
        let clear = br#"{"hello":"world"}"#;
        let encryption = Encryption::new([1; 32]);
        let out = encryption.decrypt_payload(clear).unwrap();
        assert_eq!(&out[..], &clear[..]);
    }

    #[test]
    fn test_payload_wrong_key_fails() {
        let clear = br#"{"hello":"world"}"#;
        let ciphertext = Encryption::new([1; 32]).encrypt_payload(clear).unwrap();
        assert!(
            Encryption::new([2; 32])
                .decrypt_payload(&ciphertext)
                .is_err()
        );
    }

    #[test]
    fn test_payload_noop_cant_decrypt() {
        let clear = br#"{"hello":"world"}"#;
        let ciphertext = Encryption::new([1; 32]).encrypt_payload(clear).unwrap();
        assert!(Encryption::new_noop().decrypt_payload(&ciphertext).is_err());
    }
}
