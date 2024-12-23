//! Wrapper types around those in the SDK so they can act as args in the `clap` command tree.

use clap::Args;
use svix::api;

pub mod application;
pub mod endpoint;
pub mod event_type;
pub mod integration;
pub mod message;
pub mod message_attempt;

#[derive(Args, Clone, Default)]
pub struct PostOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<PostOptions> for api::PostOptions {
    fn from(PostOptions { idempotency_key }: PostOptions) -> Self {
        Self { idempotency_key }
    }
}
