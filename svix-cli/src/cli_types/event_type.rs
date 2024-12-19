use clap::Args;
use svix::api;

#[derive(Args, Clone)]
pub struct EventTypeListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    // FIXME: This is missing from the Rust lib
    // /// The sorting order of the returned items
    // #[arg(long)]
    // pub order: Option<Ordering>,
    /// When `true` archived (deleted but not expunged) items are included in the response
    #[arg(long)]
    pub include_archived: Option<bool>,
    /// When `true` the full item (including the schema) is included in the response
    #[arg(long)]
    pub with_content: Option<bool>,
}

impl From<EventTypeListOptions> for api::EventTypeListOptions {
    fn from(
        EventTypeListOptions {
            limit,
            iterator,
            include_archived,
            with_content,
        }: EventTypeListOptions,
    ) -> Self {
        Self {
            limit,
            iterator,
            include_archived,
            with_content,
        }
    }
}
