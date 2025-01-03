//! Wrapper types around those in the SDK so they can act as args in the `clap` command tree.

use clap::Args;
use svix::api;

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
