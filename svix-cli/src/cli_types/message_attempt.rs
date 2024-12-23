use chrono::{DateTime, Utc};
use clap::Args;
use svix::{
    api,
    api::{MessageStatus, StatusCodeClass},
};

#[derive(Args, Clone)]
pub struct MessageAttemptListByEndpointOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    #[arg(long)]
    pub status: Option<MessageStatus>,
    /// Filter response based on the HTTP status code
    #[arg(long)]
    pub status_code_class: Option<StatusCodeClass>,
    /// Filter response based on the channel
    #[arg(long)]
    pub channel: Option<String>,
    /// Filter response based on the tag
    #[arg(long)]
    pub tag: Option<String>,
    /// Only include items created before a certain date
    #[arg(long)]
    pub before: Option<DateTime<Utc>>,
    /// Only include items created after a certain date
    #[arg(long)]
    pub after: Option<DateTime<Utc>>,
    /// When `true` attempt content is included in the response
    #[arg(long)]
    pub with_content: Option<bool>,
    /// When `true`, the message information is included in the response
    #[arg(long)]
    pub with_msg: Option<bool>,
    /// Filter response based on the event type
    #[arg(long)]
    pub event_types: Option<Vec<String>>,
}

impl From<MessageAttemptListByEndpointOptions> for api::MessageAttemptListByEndpointOptions {
    fn from(
        MessageAttemptListByEndpointOptions {
            limit,
            iterator,
            status,
            status_code_class,
            channel,
            tag,
            before,
            after,
            with_content,
            with_msg,
            event_types,
        }: MessageAttemptListByEndpointOptions,
    ) -> Self {
        Self {
            limit,
            iterator,
            status,
            status_code_class,
            channel,
            tag,
            before: before.map(|dt| dt.to_rfc3339()),
            after: after.map(|dt| dt.to_rfc3339()),
            with_content,
            with_msg,
            event_types,
        }
    }
}

#[derive(Args, Clone)]
pub struct MessageAttemptCountByEndpointOptions {
    /// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    #[arg(long)]
    pub status: Option<MessageStatus>,
    /// Filter response based on the HTTP status code
    #[arg(long)]
    pub status_code_class: Option<StatusCodeClass>,
    /// Filter response based on the channel
    #[arg(long)]
    pub channel: Option<String>,
    /// Filter response based on the tag
    #[arg(long)]
    pub tag: Option<String>,
    /// Only include items created before a certain date
    #[arg(long)]
    pub before: Option<DateTime<Utc>>,
    /// Only include items created after a certain date
    #[arg(long)]
    pub after: Option<DateTime<Utc>>,
    /// Filter response based on the event type
    #[arg(long)]
    pub event_types: Option<Vec<String>>,
}

#[derive(Args, Clone)]
pub struct MessageAttemptListByMsgOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    #[arg(long)]
    pub status: Option<MessageStatus>,
    /// Filter response based on the HTTP status code
    #[arg(long)]
    pub status_code_class: Option<StatusCodeClass>,
    /// Filter response based on the channel
    #[arg(long)]
    pub channel: Option<String>,
    /// Filter response based on the tag
    #[arg(long)]
    pub tag: Option<String>,
    /// Filter the attempts based on the attempted endpoint
    #[arg(long)]
    pub endpoint_id: Option<String>,
    /// Only include items created before a certain date
    #[arg(long)]
    pub before: Option<DateTime<Utc>>,
    /// Only include items created after a certain date
    #[arg(long)]
    pub after: Option<DateTime<Utc>>,
    /// When `true` attempt content is included in the response
    #[arg(long)]
    pub with_content: Option<bool>,
    /// Filter response based on the event type
    #[arg(long)]
    pub event_types: Option<Vec<String>>,
}

impl From<MessageAttemptListByMsgOptions> for api::MessageAttemptListByMsgOptions {
    fn from(
        MessageAttemptListByMsgOptions {
            limit,
            iterator,
            status,
            status_code_class,
            channel,
            tag,
            endpoint_id,
            before,
            after,
            with_content,
            event_types,
        }: MessageAttemptListByMsgOptions,
    ) -> Self {
        Self {
            limit,
            iterator,
            status,
            status_code_class,
            channel,
            tag,
            endpoint_id,
            before: before.map(|dt| dt.to_rfc3339()),
            after: after.map(|dt| dt.to_rfc3339()),
            with_content,
            event_types,
        }
    }
}

#[derive(Args, Clone)]
pub struct MessageAttemptListAttemptedMessagesOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// Filter response based on the channel
    #[arg(long)]
    pub channel: Option<String>,
    /// Filter response based on the message tags
    #[arg(long)]
    pub tag: Option<String>,
    /// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    #[arg(long)]
    pub status: Option<MessageStatus>,
    /// Only include items created before a certain date
    #[arg(long)]
    pub before: Option<DateTime<Utc>>,
    /// Only include items created after a certain date
    #[arg(long)]
    pub after: Option<DateTime<Utc>>,
    /// When `true` message payloads are included in the response
    #[arg(long)]
    pub with_content: Option<bool>,
    /// Filter response based on the event type
    #[arg(long)]
    pub event_types: Option<Vec<String>>,
}

impl From<MessageAttemptListAttemptedMessagesOptions>
    for api::MessageAttemptListAttemptedMessagesOptions
{
    fn from(
        MessageAttemptListAttemptedMessagesOptions {
            limit,
            iterator,
            channel,
            tag,
            status,
            before,
            after,
            with_content,
            event_types,
        }: MessageAttemptListAttemptedMessagesOptions,
    ) -> Self {
        Self {
            limit,
            iterator,
            channel,
            tag,
            status,
            before: before.map(|dt| dt.to_rfc3339()),
            after: after.map(|dt| dt.to_rfc3339()),
            with_content,
            event_types,
        }
    }
}

#[derive(Args, Clone)]
pub struct MessageAttemptListAttemptedDestinationsOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
}

impl From<MessageAttemptListAttemptedDestinationsOptions>
    for api::MessageAttemptListAttemptedDestinationsOptions
{
    fn from(
        MessageAttemptListAttemptedDestinationsOptions {
                    limit,iterator,
                }: MessageAttemptListAttemptedDestinationsOptions,
    ) -> Self {
        Self { limit, iterator }
    }
}
