// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

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
pub struct MessagePrecheckOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<MessagePrecheckOptions> for svix::api::MessagePrecheckOptions {
    fn from(value: MessagePrecheckOptions) -> Self {
        let MessagePrecheckOptions { idempotency_key } = value;
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

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MessageArgs {
    #[command(subcommand)]
    pub command: MessageCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum MessageCommands {
    Poller(MessagePollerArgs),
    /// List all of the application's messages.
    ///
    /// The `before` and `after` parameters let you filter all items created before or after a certain date. These can be
    /// used alongside an iterator to paginate over results within a certain window.
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
    /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
    /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message list app_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"channels\": [\"project_123\",\"group_2\"],
    \"deliverAt\": \"2030-01-01T00:00:00Z\",
    \"eventId\": \"unique-identifier\",
    \"eventType\": \"user.signup\",
    \"id\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"payload\": {
      \"email\": \"test@example.com\",
      \"type\": \"user.created\",
      \"username\": \"test_user\"
    },
    \"tags\": [\"...\"],
    \"timestamp\": \"2030-01-01T00:00:00Z\"
  }],
  \"done\": true,
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\"
}\n")]
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
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message create app_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"application\": {
    \"metadata\": {\"key\": \"...\"},
    \"name\": \"My first application\",
    \"throttleRate\": 123,
    \"uid\": \"unique-identifier\"
  },
  \"channels\": [\"project_123\",\"group_2\"],
  \"deliverAt\": \"2030-01-01T00:00:00Z\",
  \"eventId\": \"unique-identifier\",
  \"eventType\": \"user.signup\",
  \"payload\": {
    \"email\": \"test@example.com\",
    \"type\": \"user.created\",
    \"username\": \"test_user\"
  },
  \"payloadRetentionHours\": 123,
  \"payloadRetentionPeriod\": 90,
  \"tags\": [\"my_tag\",\"other\"],
  \"transformationsParams\": {\"key\": \"...\"}
}\n\nExample response:
{
  \"channels\": [\"project_123\",\"group_2\"],
  \"deliverAt\": \"2030-01-01T00:00:00Z\",
  \"eventId\": \"unique-identifier\",
  \"eventType\": \"user.signup\",
  \"id\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"payload\": {
    \"email\": \"test@example.com\",
    \"type\": \"user.created\",
    \"username\": \"test_user\"
  },
  \"tags\": [\"...\"],
  \"timestamp\": \"2030-01-01T00:00:00Z\"
}\n")]
    Create {
        app_id: String,
        message_in: crate::json::JsonOf<MessageIn>,
        #[clap(flatten)]
        options: MessageCreateOptions,
    },
    /// A pre-check call for `message.create` that checks whether any active endpoints are
    /// listening to this message.
    ///
    /// Note: most people shouldn't be using this API. Svix doesn't bill you for
    /// messages not actually sent, so using this API doesn't save money.
    /// If unsure, please ask Svix support before using this API.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message precheck app_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"channels\": [\"project_123\",\"group_2\"],
  \"eventType\": \"user.signup\"
}\n\nExample response:
{
  \"active\": true
}\n")]
    Precheck {
        app_id: String,
        message_precheck_in: crate::json::JsonOf<MessagePrecheckIn>,
        #[clap(flatten)]
        options: MessagePrecheckOptions,
    },
    /// Get a message by its ID or eventID.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message get app_abc000000000000000000000000 msg_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"channels\": [\"project_123\",\"group_2\"],
  \"deliverAt\": \"2030-01-01T00:00:00Z\",
  \"eventId\": \"unique-identifier\",
  \"eventType\": \"user.signup\",
  \"id\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"payload\": {
    \"email\": \"test@example.com\",
    \"type\": \"user.created\",
    \"username\": \"test_user\"
  },
  \"tags\": [\"...\"],
  \"timestamp\": \"2030-01-01T00:00:00Z\"
}\n")]
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
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message expunge-content app_abc000000000000000000000000 msg_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    ExpungeContent {
        app_id: String,
        id: String,
    },
    /// Delete all message payloads for the application.
    ///
    /// This operation is only available in the <a href="https://svix.com/pricing" target="_blank">Enterprise</a> plan.
    ///
    /// A completed task will return a payload like the following:
    /// ```json
    /// {
    ///   "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
    ///   "status": "finished",
    ///   "task": "application.purge_content",
    ///   "data": {
    ///     "messagesPurged": 150
    ///   }
    /// }
    /// ```
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message expunge-all-contents app_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"id\": \"qtask_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"status\": \"running\",
  \"task\": \"endpoint.replay\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\"
}\n")]
    ExpungeAllContents {
        app_id: String,
        #[clap(flatten)]
        options: MessageExpungeAllContentsOptions,
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
            Self::Precheck {
                app_id,
                message_precheck_in,
                options,
            } => {
                let resp = client
                    .message()
                    .precheck(
                        app_id,
                        message_precheck_in.into_inner(),
                        Some(options.into()),
                    )
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
            Self::ExpungeAllContents { app_id, options } => {
                let resp = client
                    .message()
                    .expunge_all_contents(app_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
