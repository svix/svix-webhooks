// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

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
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    /// Only include items created after a certain date
    #[arg(long)]
    pub after: Option<chrono::DateTime<chrono::Utc>>,
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

impl From<MessageAttemptListByEndpointOptions> for svix::api::MessageAttemptListByEndpointOptions {
    fn from(value: MessageAttemptListByEndpointOptions) -> Self {
        let MessageAttemptListByEndpointOptions {
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
        } = value;
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
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    /// Only include items created after a certain date
    #[arg(long)]
    pub after: Option<chrono::DateTime<chrono::Utc>>,
    /// When `true` attempt content is included in the response
    #[arg(long)]
    pub with_content: Option<bool>,
    /// Filter response based on the event type
    #[arg(long)]
    pub event_types: Option<Vec<String>>,
}

impl From<MessageAttemptListByMsgOptions> for svix::api::MessageAttemptListByMsgOptions {
    fn from(value: MessageAttemptListByMsgOptions) -> Self {
        let MessageAttemptListByMsgOptions {
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
        } = value;
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
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    /// Only include items created after a certain date
    #[arg(long)]
    pub after: Option<chrono::DateTime<chrono::Utc>>,
    /// When `true` message payloads are included in the response
    #[arg(long)]
    pub with_content: Option<bool>,
    /// Filter response based on the event type
    #[arg(long)]
    pub event_types: Option<Vec<String>>,
}

impl From<MessageAttemptListAttemptedMessagesOptions>
    for svix::api::MessageAttemptListAttemptedMessagesOptions
{
    fn from(value: MessageAttemptListAttemptedMessagesOptions) -> Self {
        let MessageAttemptListAttemptedMessagesOptions {
            limit,
            iterator,
            channel,
            tag,
            status,
            before,
            after,
            with_content,
            event_types,
        } = value;
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
    for svix::api::MessageAttemptListAttemptedDestinationsOptions
{
    fn from(value: MessageAttemptListAttemptedDestinationsOptions) -> Self {
        let MessageAttemptListAttemptedDestinationsOptions { limit, iterator } = value;
        Self { limit, iterator }
    }
}

#[derive(Args, Clone)]
pub struct MessageAttemptResendOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<MessageAttemptResendOptions> for svix::api::MessageAttemptResendOptions {
    fn from(value: MessageAttemptResendOptions) -> Self {
        let MessageAttemptResendOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MessageAttemptArgs {
    #[command(subcommand)]
    pub command: MessageAttemptCommands,
}

#[derive(Subcommand)]
pub enum MessageAttemptCommands {
    /// List attempts by endpoint id
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
    /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
    /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
    ListByEndpoint {
        app_id: String,
        endpoint_id: String,
        #[clap(flatten)]
        options: MessageAttemptListByEndpointOptions,
    },
    /// List attempts by message ID.
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
    /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
    /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
    ListByMsg {
        app_id: String,
        msg_id: String,
        #[clap(flatten)]
        options: MessageAttemptListByMsgOptions,
    },
    /// List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.
    ///
    /// The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
    /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
    /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
    ListAttemptedMessages {
        app_id: String,
        endpoint_id: String,
        #[clap(flatten)]
        options: MessageAttemptListAttemptedMessagesOptions,
    },
    /// `msg_id`: Use a message id or a message `eventId`
    Get {
        app_id: String,
        msg_id: String,
        attempt_id: String,
    },
    /// Deletes the given attempt's response body.
    ///
    /// Useful when an endpoint accidentally returned sensitive content.
    /// The message can't be replayed or resent once its payload has been deleted or expired.
    ExpungeContent {
        app_id: String,
        msg_id: String,
        attempt_id: String,
    },
    /// List endpoints attempted by a given message.
    ///
    /// Additionally includes metadata about the latest message attempt.
    /// By default, endpoints are listed in ascending order by ID.
    ListAttemptedDestinations {
        app_id: String,
        msg_id: String,
        #[clap(flatten)]
        options: MessageAttemptListAttemptedDestinationsOptions,
    },
    /// Resend a message to the specified endpoint.
    Resend {
        app_id: String,
        msg_id: String,
        endpoint_id: String,
        #[clap(flatten)]
        options: MessageAttemptResendOptions,
    },
}

impl MessageAttemptCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::ListByEndpoint {
                app_id,
                endpoint_id,
                options,
            } => {
                let resp = client
                    .message_attempt()
                    .list_by_endpoint(app_id, endpoint_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ListByMsg {
                app_id,
                msg_id,
                options,
            } => {
                let resp = client
                    .message_attempt()
                    .list_by_msg(app_id, msg_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ListAttemptedMessages {
                app_id,
                endpoint_id,
                options,
            } => {
                let resp = client
                    .message_attempt()
                    .list_attempted_messages(app_id, endpoint_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                app_id,
                msg_id,
                attempt_id,
            } => {
                let resp = client
                    .message_attempt()
                    .get(app_id, msg_id, attempt_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ExpungeContent {
                app_id,
                msg_id,
                attempt_id,
            } => {
                client
                    .message_attempt()
                    .expunge_content(app_id, msg_id, attempt_id)
                    .await?;
            }
            Self::ListAttemptedDestinations {
                app_id,
                msg_id,
                options,
            } => {
                let resp = client
                    .message_attempt()
                    .list_attempted_destinations(app_id, msg_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Resend {
                app_id,
                msg_id,
                endpoint_id,
                options,
            } => {
                client
                    .message_attempt()
                    .resend(app_id, msg_id, endpoint_id, Some(options.into()))
                    .await?;
            }
        }

        Ok(())
    }
}
