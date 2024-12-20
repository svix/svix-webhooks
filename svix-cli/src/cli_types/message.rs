use chrono::{DateTime, Utc};
use clap::Args;
use svix::api;

#[derive(Args, Clone)]
pub struct MessageListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// Filter response based on the channel
    #[arg(long)]
    pub channel: Option<String>,
    /// Only include items created before a certain date
    #[arg(long)]
    pub before: Option<DateTime<Utc>>,
    /// Only include items created after a certain date
    #[arg(long)]
    pub after: Option<DateTime<Utc>>,
    /// When `true` message payloads are included in the response
    #[arg(long)]
    pub with_content: Option<bool>,
    /// Filter messages matching the provided tag
    #[arg(long)]
    pub tag: Option<String>,
    /// Filter response based on the event type
    #[arg(long)]
    pub event_types: Option<Vec<String>>,
}

impl From<MessageListOptions> for api::MessageListOptions {
    fn from(
        MessageListOptions {
            limit,
            iterator,
            channel,
            before,
            after,
            with_content,
            tag,
            event_types,
        }: MessageListOptions,
    ) -> Self {
        Self {
            limit,
            iterator,
            channel,
            before: before.map(|dt| dt.to_rfc3339()),
            after: after.map(|dt| dt.to_rfc3339()),
            with_content,
            tag,
            event_types,
        }
    }
}
