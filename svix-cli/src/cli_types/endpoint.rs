use crate::cli_types::Ordering;
use chrono::{DateTime, Utc};
use clap::Args;
use svix::api;

#[derive(Args, Clone)]
pub struct EndpointListOptions {
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

impl From<EndpointListOptions> for api::EndpointListOptions {
    fn from(
        EndpointListOptions {
            limit,
            iterator,
            order,
        }: EndpointListOptions,
    ) -> Self {
        Self {
            limit,
            iterator,
            order: order.map(Into::into),
        }
    }
}

#[derive(Args, Clone)]
pub struct EndpointStatsOptions {
    /// Filter the range to data ending by this date
    #[arg(long)]
    pub before: Option<DateTime<Utc>>,
    /// Filter the range to data starting from this date
    #[arg(long)]
    pub after: Option<DateTime<Utc>>,
}

impl From<EndpointStatsOptions> for api::EndpointStatsOptions {
    fn from(EndpointStatsOptions { after, before }: EndpointStatsOptions) -> Self {
        Self {
            until: before.map(|dt| dt.to_rfc3339()),
            since: after.map(|dt| dt.to_rfc3339()),
        }
    }
}
