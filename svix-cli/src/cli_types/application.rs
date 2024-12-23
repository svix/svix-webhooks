use clap::Args;
use svix::{api, api::Ordering};

#[derive(Args, Clone)]
pub struct ApplicationListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// The sorting order of the returned items
    #[arg(long)]
    pub order: Option<Ordering>,
}

impl From<ApplicationListOptions> for api::ApplicationListOptions {
    fn from(
        ApplicationListOptions {
            limit,
            iterator,
            order,
        }: ApplicationListOptions,
    ) -> Self {
        Self {
            limit,
            iterator,

            order: order.map(Into::into),
        }
    }
}
