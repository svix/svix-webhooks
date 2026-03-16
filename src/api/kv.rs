// this file is @generated
use super::KvNamespace;
use crate::{Configuration, error::Result, models::*};

pub struct Kv<'a> {
    cfg: &'a Configuration,
}

impl<'a> Kv<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn namespace(&self) -> KvNamespace<'a> {
        KvNamespace::new(self.cfg)
    }

    /// KV Set
    pub async fn set(&self, key: String, kv_set_in: KvSetIn) -> Result<KvSetOut> {
        let kv_set_in = KvSetIn_ {
            key,
            value: kv_set_in.value,
            ttl: kv_set_in.ttl,
            behavior: kv_set_in.behavior,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1/kv/set")
            .with_body(kv_set_in)
            .execute(self.cfg)
            .await
    }

    /// KV Get
    pub async fn get(&self, key: String, kv_get_in: KvGetIn) -> Result<KvGetOut> {
        let kv_get_in = KvGetIn_ {
            key,
            consistency: kv_get_in.consistency,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1/kv/get")
            .with_body(kv_get_in)
            .execute(self.cfg)
            .await
    }

    /// KV Delete
    pub async fn delete(&self, key: String, kv_delete_in: KvDeleteIn) -> Result<KvDeleteOut> {
        let _unused = kv_delete_in;
        let kv_delete_in = KvDeleteIn_ { key };

        crate::request::Request::new(http::Method::POST, "/api/v1/kv/delete")
            .with_body(kv_delete_in)
            .execute(self.cfg)
            .await
    }
}
