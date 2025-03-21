use clap::{Args, Subcommand};
use svix::api::*;

use crate::json::JsonOf;

#[derive(Args, Clone)]
pub struct EventsPublicConsumerOptions {
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
}

impl From<EventsPublicConsumerOptions> for svix::api::EventsPublicConsumerOptions {
    fn from(value: EventsPublicConsumerOptions) -> Self {
        let EventsPublicConsumerOptions {
            limit,
            iterator,
            event_type,
            channel,
        } = value;
        Self {
            limit,
            iterator,
            event_type,
            channel,
        }
    }
}

#[derive(Args, Clone)]
pub struct EventsPublicConsumerSeekOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EventsPublicConsumerSeekOptions> for svix::api::EventsPublicConsumerSeekOptions {
    fn from(value: EventsPublicConsumerSeekOptions) -> Self {
        let EventsPublicConsumerSeekOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct EventsPublicArgs {
    #[command(subcommand)]
    pub command: EventsPublicCommands,
}

#[derive(Subcommand)]
pub enum EventsPublicCommands {
    /// Reads the stream of created messages for an application, filtered on the Sink's event types and
    /// Channels, using server-managed iterator tracking.
    Consumer {
        app_id: String,
        sink_id: String,
        consumer_id: String,
        #[clap(flatten)]
        options: EventsPublicConsumerOptions,
    },
    /// Sets the starting offset for the consumer of a polling endpoint.
    ConsumerSeek {
        app_id: String,
        sink_id: String,
        consumer_id: String,
        polling_endpoint_consumer_seek_in: JsonOf<PollingEndpointConsumerSeekIn>,
        #[clap(flatten)]
        options: EventsPublicConsumerSeekOptions,
    },
}

impl EventsPublicCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Consumer {
                app_id,
                sink_id,
                consumer_id,
                options,
            } => {
                let resp = client
                    .events_public()
                    .consumer(app_id, sink_id, consumer_id, Some(options.into()))
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
                    .events_public()
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
