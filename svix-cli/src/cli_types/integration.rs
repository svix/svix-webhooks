use crate::cli_types::Ordering;
use clap::Args;
use svix::api;

#[derive(Args, Clone)]
pub struct IntegrationListOptions {
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

impl From<IntegrationListOptions> for api::IntegrationListOptions {
    fn from(
        IntegrationListOptions {
            limit,
            iterator,
            order,
        }: IntegrationListOptions,
    ) -> Self {
        Self {
            limit,
            iterator,

            order: order.map(Into::into),
        }
    }
}
