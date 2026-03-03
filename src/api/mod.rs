// this file is @generated
use crate::CoyoteClient;

mod cache;
mod health;
mod idempotency;
mod kv;
mod msgs;
mod msgs_namespace;
mod msgs_stream;
mod msgs_topic;
mod rate_limiter;

pub use self::{
    cache::Cache, health::Health, idempotency::Idempotency, kv::Kv, msgs::Msgs,
    msgs_namespace::MsgsNamespace, msgs_stream::MsgsStream, msgs_topic::MsgsTopic,
    rate_limiter::RateLimiter,
};

impl CoyoteClient {
    pub fn cache(&self) -> Cache<'_> {
        Cache::new(&self.cfg)
    }

    pub fn health(&self) -> Health<'_> {
        Health::new(&self.cfg)
    }

    pub fn idempotency(&self) -> Idempotency<'_> {
        Idempotency::new(&self.cfg)
    }

    pub fn kv(&self) -> Kv<'_> {
        Kv::new(&self.cfg)
    }

    pub fn msgs(&self) -> Msgs<'_> {
        Msgs::new(&self.cfg)
    }

    pub fn rate_limiter(&self) -> RateLimiter<'_> {
        RateLimiter::new(&self.cfg)
    }
}
