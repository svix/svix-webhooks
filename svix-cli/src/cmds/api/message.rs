use clap::{Args, Subcommand};
use svix::api::*;

use super::message_poller::MessagePollerArgs;

#[derive(Args, Clone)]
pub struct MessageListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// Filter response based on the channel.
    #[arg(long)]
    pub channel: Option<String>,
    /// Only include items created before a certain date.
    #[arg(long)]
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    /// Only include items created after a certain date.
    #[arg(long)]
    pub after: Option<chrono::DateTime<chrono::Utc>>,
    /// When `true` message payloads are included in the response.
    #[arg(long)]
    pub with_content: Option<bool>,
    /// Filter messages matching the provided tag.
    #[arg(long)]
    pub tag: Option<String>,
    /// Filter response based on the event type
    #[arg(long)]
    pub event_types: Option<Vec<String>>,
}

impl From<MessageListOptions> for svix::api::MessageListOptions {
    fn from(value: MessageListOptions) -> Self {
        let MessageListOptions {
            limit,
            iterator,
            channel,
            before,
            after,
            with_content,
            tag,
            event_types,
        } = value;
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

#[derive(Args, Clone)]
pub struct MessageCreateOptions {
    /// When `true`, message payloads are included in the response.
    #[arg(long)]
    pub with_content: Option<bool>,

    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<MessageCreateOptions> for svix::api::MessageCreateOptions {
    fn from(value: MessageCreateOptions) -> Self {
        let MessageCreateOptions {
            with_content,
            idempotency_key,
        } = value;
        Self {
            with_content,
            idempotency_key,
        }
    }
}

#[derive(Args, Clone)]
pub struct MessageExpungeAllContentsOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<MessageExpungeAllContentsOptions> for svix::api::MessageExpungeAllContentsOptions {
    fn from(value: MessageExpungeAllContentsOptions) -> Self {
        let MessageExpungeAllContentsOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct MessageGetOptions {
    /// When `true` message payloads are included in the response.
    #[arg(long)]
    pub with_content: Option<bool>,
}

impl From<MessageGetOptions> for svix::api::MessageGetOptions {
    fn from(value: MessageGetOptions) -> Self {
        let MessageGetOptions { with_content } = value;
        Self { with_content }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MessageArgs {
    #[command(subcommand)]
    pub command: MessageCommands,
}

#[derive(Subcommand)]
pub enum MessageCommands {
    Poller(MessagePollerArgs),
    /// List all of the application's messages.
    ///
    /// The `before` and `after` parameters let you filter all items created before or after a certain date. These can be used alongside an iterator to paginate over results
    /// within a certain window.
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
    /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
    /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
    List {
        app_id: String,
        #[clap(flatten)]
        options: MessageListOptions,
    },
    /// Creates a new message and dispatches it to all of the application's endpoints.
    ///
    /// The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made.
    /// If a message with the same `eventId` already exists for the application, a 409 conflict error will be returned.
    ///
    /// The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types.
    /// Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.
    ///
    /// The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to 1MiB, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
    Create {
        app_id: String,
        message_in: crate::json::JsonOf<MessageIn>,
        #[clap(flatten)]
        options: MessageCreateOptions,
    },
    /// Delete all message payloads for the application.
    ///
    /// This operation is only available in the <a href="https://svix.com/pricing" target="_blank">Enterprise</a> plan.
    ExpungeAllContents {
        app_id: String,
        #[clap(flatten)]
        options: MessageExpungeAllContentsOptions,
    },
    /// Get a message by its ID or eventID.
    Get {
        app_id: String,
        id: String,
        #[clap(flatten)]
        options: MessageGetOptions,
    },
    /// Delete the given message's payload.
    ///
    /// Useful in cases when a message was accidentally sent with sensitive content.
    /// The message can't be replayed or resent once its payload has been deleted or expired.
    ExpungeContent {
        app_id: String,
        id: String,
    },
}

impl MessageCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Poller(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::List { app_id, options } => {
                let resp = client.message().list(app_id, Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                app_id,
                message_in,
                options,
            } => {
                let resp = client
                    .message()
                    .create(app_id, message_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ExpungeAllContents { app_id, options } => {
                let resp = client
                    .message()
                    .expunge_all_contents(app_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                app_id,
                id,
                options,
            } => {
                let resp = client
                    .message()
                    .get(app_id, id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ExpungeContent { app_id, id } => {
                client.message().expunge_content(app_id, id).await?;
            }
        }

        Ok(())
    }
}
