// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args, Clone)]
pub struct MessagePollerPollOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// Filters messages sent with this event type (optional).
    #[arg(long)]
    pub event_type: Option<String>,
    /// Filters messages sent with this channel (optional).
    #[arg(long)]
    pub channel: Option<String>,
    #[arg(long)]
    pub after: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<MessagePollerPollOptions> for svix::api::MessagePollerPollOptions {
    fn from(value: MessagePollerPollOptions) -> Self {
        let MessagePollerPollOptions {
            limit,
            iterator,
            event_type,
            channel,
            after,
        } = value;
        Self {
            limit,
            iterator,
            event_type,
            channel,
            after: after.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Args, Clone)]
pub struct MessagePollerConsumerSeekOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<MessagePollerConsumerSeekOptions> for svix::api::MessagePollerConsumerSeekOptions {
    fn from(value: MessagePollerConsumerSeekOptions) -> Self {
        let MessagePollerConsumerSeekOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct MessagePollerConsumerPollOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
}

impl From<MessagePollerConsumerPollOptions> for svix::api::MessagePollerConsumerPollOptions {
    fn from(value: MessagePollerConsumerPollOptions) -> Self {
        let MessagePollerConsumerPollOptions { limit, iterator } = value;
        Self { limit, iterator }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MessagePollerArgs {
    #[command(subcommand)]
    pub command: MessagePollerCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum MessagePollerCommands {
    /// Reads the stream of created messages for an application, filtered on the Sink's event types and Channels.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message poller poll app_abc000000000000000000000000 sink_abc000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"headers\": {\"key\": \"...\"},
    \"eventId\": \"unique-identifier\",
    \"eventType\": \"user.signup\",
    \"payload\": {
      \"email\": \"test@example.com\",
      \"type\": \"user.created\",
      \"username\": \"test_user\"
    },
    \"channels\": [\"project_123\",\"group_2\"],
    \"id\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"timestamp\": \"2030-01-01T00:00:00Z\",
    \"tags\": [\"...\"],
    \"deliverAt\": \"2030-01-01T00:00:00Z\"
  }],
  \"iterator\": \"...\",
  \"done\": true
}\n")]
    Poll {
        app_id: String,
        sink_id: String,
        #[clap(flatten)]
        options: MessagePollerPollOptions,
    },
    /// Sets the starting offset for the consumer of a polling endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message poller consumer-seek app_abc000000000000000000000000 sink_abc000000000000000000 CONSUMER_ID {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"after\": \"2025-04-21T11:20:34Z\"
}\n\nExample response:
{
  \"iterator\": \"...\"
}\n")]
    ConsumerSeek {
        app_id: String,
        sink_id: String,
        consumer_id: String,
        polling_endpoint_consumer_seek_in: crate::json::JsonOf<PollingEndpointConsumerSeekIn>,
        #[clap(flatten)]
        options: MessagePollerConsumerSeekOptions,
    },
    /// Reads the stream of created messages for an application, filtered on the Sink's event types and
    /// Channels, using server-managed iterator tracking.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message poller consumer-poll app_abc000000000000000000000000 sink_abc000000000000000000 CONSUMER_ID\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"headers\": {\"key\": \"...\"},
    \"eventId\": \"unique-identifier\",
    \"eventType\": \"user.signup\",
    \"payload\": {
      \"email\": \"test@example.com\",
      \"type\": \"user.created\",
      \"username\": \"test_user\"
    },
    \"channels\": [\"project_123\",\"group_2\"],
    \"id\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"timestamp\": \"2030-01-01T00:00:00Z\",
    \"tags\": [\"...\"],
    \"deliverAt\": \"2030-01-01T00:00:00Z\"
  }],
  \"iterator\": \"...\",
  \"done\": true
}\n")]
    ConsumerPoll {
        app_id: String,
        sink_id: String,
        consumer_id: String,
        #[clap(flatten)]
        options: MessagePollerConsumerPollOptions,
    },
}

impl MessagePollerCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Poll {
                app_id,
                sink_id,
                options,
            } => {
                let resp = client
                    .message()
                    .poller()
                    .poll(app_id, sink_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ConsumerSeek {
                app_id,
                sink_id,
                consumer_id,
                polling_endpoint_consumer_seek_in,
                options,
            } => {
                let resp = client
                    .message()
                    .poller()
                    .consumer_seek(
                        app_id,
                        sink_id,
                        consumer_id,
                        polling_endpoint_consumer_seek_in.into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ConsumerPoll {
                app_id,
                sink_id,
                consumer_id,
                options,
            } => {
                let resp = client
                    .message()
                    .poller()
                    .consumer_poll(app_id, sink_id, consumer_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
