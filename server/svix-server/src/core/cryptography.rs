// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::fmt::Debug;

use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use ed25519_compact::*;
use rand::Rng;

use crate::err_generic;
use crate::error::Result;

// Asymmetric Signature keys
#[derive(Clone, Eq)]
pub struct AsymmetricKey(pub KeyPair);

impl AsymmetricKey {
    pub fn generate() -> AsymmetricKey {
        AsymmetricKey(KeyPair::from_seed(Seed::generate()))
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        Ok(AsymmetricKey(
            KeyPair::from_slice(bytes).map_err(|_| err_generic!("Failed parsing key."))?,
        ))
    }

    pub fn from_base64(b64: &str) -> Result<Self> {
        let bytes = base64::decode(b64).map_err(|_| err_generic!("Failed parsing base64"))?;

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
            base64::encode(self.0.pk.as_slice())
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

    pub fn new_noop() -> Self {
        Self(None)
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self(Some(Key::from_slice(&key).to_owned()))
    }

    pub fn encrypt(&self, data: &[u8]) -> crate::error::Result<Vec<u8>> {
        if let Some(main_key) = self.0.as_ref() {
            let cipher = XChaCha20Poly1305::new(main_key);
            let nonce: [u8; Self::NONCE_SIZE] = rand::thread_rng().gen();
            let nonce = XNonce::from_slice(&nonce);
            let mut ciphertext = cipher
                .encrypt(nonce, data)
                .map_err(|_| err_generic!("Encryption failed"))?;
            let mut ret = nonce.to_vec();
            ret.append(&mut ciphertext);
            Ok(ret)
        } else {
            Ok(data.to_vec())
        }
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> crate::error::Result<Vec<u8>> {
        if let Some(main_key) = self.0.as_ref() {
            let cipher = XChaCha20Poly1305::new(main_key);
            let nonce = &ciphertext[..Self::NONCE_SIZE];
            let ciphertext = &ciphertext[Self::NONCE_SIZE..];
            cipher
                .decrypt(XNonce::from_slice(nonce), ciphertext)
                .map_err(|_| err_generic!("Encryption failed"))
        } else {
            Ok(ciphertext.to_vec())
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
    use super::*;

    #[test]
    fn test_encryption() {
        let clear = b"hello world";
        let encryption = Encryption::new([1; 32]);
        let ciphertext = encryption.encrypt(clear).unwrap();
        let clear2 = encryption.decrypt(&ciphertext).unwrap();
        assert_eq!(&clear[..], &clear2[..]);
    }
}
