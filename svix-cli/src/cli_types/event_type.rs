use clap::Args;
use svix::{api, api::Ordering};

#[derive(Args, Clone)]
pub struct EventTypeListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// The sorting order of the returned items
    #[arg(long)]
    pub order: Option<Ordering>,
    /// When `true` archived (deleted but not expunged) items are included in the response.
    #[arg(long)]
    pub include_archived: Option<bool>,
    /// When `true` the full item (including the schema) is included in the response.
    #[arg(long)]
    pub with_content: Option<bool>,
}

impl From<EventTypeListOptions> for api::EventTypeListOptions {
    fn from(
        EventTypeListOptions {
            limit,
            iterator,
            order,
            include_archived,
            with_content,
        }: EventTypeListOptions,
    ) -> Self {
        Self {
            limit,
            iterator,

            order: order.map(Into::into),
            include_archived,
            with_content,
        }
    }
}

// FIXME: need to get the options happening in the SDK for this
// #[derive(Args, Clone)]
// pub struct EventTypeDeleteOptions {
//     /// By default event types are archived when "deleted". Passing this to `true` deletes them entirely.
//     #[arg(long)]
//     pub expunge: Option<bool>,
// }
//
// impl From<EventTypeDeleteOptions> for api::EventTypeDeleteOptions {
//     fn from(EventTypeDeleteOptions { expunge }: EventTypeDeleteOptions) -> Self {
//         Self { expunge }
//     }
// }
