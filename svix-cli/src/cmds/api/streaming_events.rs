// this file is @generated
use clap::{
    Args,
    Subcommand,
};
use svix::api::*;

#[derive(Args, Clone)]
pub struct StreamingEventsCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamingEventsCreateOptions> for svix::api::StreamingEventsCreateOptions {
    fn from(value: StreamingEventsCreateOptions) -> Self {
        let StreamingEventsCreateOptions {
            idempotency_key,
        } = value;
        Self {
            idempotency_key,
        }
    }
}

#[derive(Args, Clone)]
pub struct StreamingEventsGetOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    #[arg(long)]
    pub after: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<StreamingEventsGetOptions> for svix::api::StreamingEventsGetOptions {
    fn from(value: StreamingEventsGetOptions) -> Self {
        let StreamingEventsGetOptions {
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
pub struct StreamingEventsArgs {
    #[command(subcommand)]
    pub command: StreamingEventsCommands,
}

#[derive(Subcommand)]
pub enum StreamingEventsCommands {
    /// Creates events on the Stream.
    Create {
        stream_id: String,
        create_stream_events_in: crate::json::JsonOf<CreateStreamEventsIn>,
        #[clap(flatten)]
        options: StreamingEventsCreateOptions,
    },
    /// Iterate over a stream of events.
    ///
    /// The sink must be of type `poller` to use the poller endpoint.
    Get {
        stream_id: String,
        sink_id: String,
        #[clap(flatten)]
        options: StreamingEventsGetOptions,
    },
}

impl StreamingEventsCommands {
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
                    .streaming()
                    .events()
                    .create(
                        stream_id,
                        create_stream_events_in.into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::Get {
                stream_id,
                sink_id,
                options,
            } => {
                let resp = client
                    .streaming()
                    .events()
                    .get(
                        stream_id,
                        sink_id,
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
        }

        Ok(())
    }
}
