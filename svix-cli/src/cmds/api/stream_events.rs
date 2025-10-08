// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct StreamEventsCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamEventsCreateOptions> for svix::api::StreamEventsCreateOptions {
    fn from(value: StreamEventsCreateOptions) -> Self {
        let StreamEventsCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct StreamEventsGetOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    #[arg(long)]
    pub after: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<StreamEventsGetOptions> for svix::api::StreamEventsGetOptions {
    fn from(value: StreamEventsGetOptions) -> Self {
        let StreamEventsGetOptions {
            limit,
            iterator,
            after,
        } = value;
        Self {
            limit,
            iterator,
            after: after.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamEventsArgs {
    #[command(subcommand)]
    pub command: StreamEventsCommands,
}

#[derive(Subcommand)]
pub enum StreamEventsCommands {
    /// Creates events on the Stream.
    Create {
        stream_id: String,
        create_stream_events_in: crate::json::JsonOf<CreateStreamEventsIn>,
        #[clap(flatten)]
        options: StreamEventsCreateOptions,
    },
    /// Iterate over a stream of events.
    ///
    /// The sink must be of type `poller` to use the poller endpoint.
    Get {
        stream_id: String,
        sink_id: String,
        #[clap(flatten)]
        options: StreamEventsGetOptions,
    },
}

impl StreamEventsCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create {
                stream_id,
                create_stream_events_in,
                options,
            } => {
                let resp = client
                    .stream()
                    .events()
                    .create(
                        stream_id,
                        create_stream_events_in.into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                stream_id,
                sink_id,
                options,
            } => {
                let resp = client
                    .stream()
                    .events()
                    .get(stream_id, sink_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
