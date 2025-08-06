use clap::{Args, Subcommand};
use svix::api::*;

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

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MessagePollerArgs {
    #[command(subcommand)]
    pub command: MessagePollerCommands,
}

#[derive(Subcommand)]
pub enum MessagePollerCommands {
    /// Reads the stream of created messages for an application, filtered on the Sink's event types and Channels.
    Poll {
        app_id: String,
        sink_id: String,
        #[clap(flatten)]
        options: MessagePollerPollOptions,
    },
    /// Reads the stream of created messages for an application, filtered on the Sink's event types and
    /// Channels, using server-managed iterator tracking.
    ConsumerPoll {
        app_id: String,
        sink_id: String,
        consumer_id: String,
        #[clap(flatten)]
        options: MessagePollerConsumerPollOptions,
    },
    /// Sets the starting offset for the consumer of a polling endpoint.
    ConsumerSeek {
        app_id: String,
        sink_id: String,
        consumer_id: String,
        polling_endpoint_consumer_seek_in: crate::json::JsonOf<PollingEndpointConsumerSeekIn>,
        #[clap(flatten)]
        options: MessagePollerConsumerSeekOptions,
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
        }

        Ok(())
    }
}
