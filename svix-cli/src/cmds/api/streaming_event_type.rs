// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct StreamingEventTypeListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// The sorting order of the returned items
    #[arg(long)]
    pub order: Option<Ordering>,
    /// Include archived (deleted but not expunged) items in the response.
    #[arg(long)]
    pub include_archived: Option<bool>,
}

impl From<StreamingEventTypeListOptions> for svix::api::StreamingEventTypeListOptions {
    fn from(value: StreamingEventTypeListOptions) -> Self {
        let StreamingEventTypeListOptions {
            limit,
            iterator,
            order,
            include_archived,
        } = value;
        Self {
            limit,
            iterator,
            order,
            include_archived,
        }
    }
}

#[derive(Args, Clone)]
pub struct StreamingEventTypeCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamingEventTypeCreateOptions> for svix::api::StreamingEventTypeCreateOptions {
    fn from(value: StreamingEventTypeCreateOptions) -> Self {
        let StreamingEventTypeCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct StreamingEventTypeDeleteOptions {
    /// By default, event types are archived when "deleted". With this flag, they are deleted entirely.
    #[arg(long)]
    pub expunge: Option<bool>,
}

impl From<StreamingEventTypeDeleteOptions> for svix::api::StreamingEventTypeDeleteOptions {
    fn from(value: StreamingEventTypeDeleteOptions) -> Self {
        let StreamingEventTypeDeleteOptions { expunge } = value;
        Self { expunge }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamingEventTypeArgs {
    #[command(subcommand)]
    pub command: StreamingEventTypeCommands,
}

#[derive(Subcommand)]
pub enum StreamingEventTypeCommands {
    /// List of all the organization's event types for streaming.
    List {
        #[clap(flatten)]
        options: StreamingEventTypeListOptions,
    },
    /// Create an event type for Streams.
    Create {
        stream_event_type_in: crate::json::JsonOf<StreamEventTypeIn>,
        #[clap(flatten)]
        options: StreamingEventTypeCreateOptions,
    },
    /// Get an event type.
    Get { name: String },
    /// Update or create a event type for Streams.
    Update {
        name: String,
        stream_event_type_in: crate::json::JsonOf<StreamEventTypeIn>,
    },
    /// Delete an event type.
    Delete {
        name: String,
        #[clap(flatten)]
        options: StreamingEventTypeDeleteOptions,
    },
    /// Patch an event type for Streams.
    Patch {
        name: String,
        stream_event_type_patch: Option<crate::json::JsonOf<StreamEventTypePatch>>,
    },
}

impl StreamingEventTypeCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client
                    .streaming()
                    .event_type()
                    .list(Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                stream_event_type_in,
                options,
            } => {
                let resp = client
                    .streaming()
                    .event_type()
                    .create(stream_event_type_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { name } => {
                let resp = client.streaming().event_type().get(name).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                name,
                stream_event_type_in,
            } => {
                let resp = client
                    .streaming()
                    .event_type()
                    .update(name, stream_event_type_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { name, options } => {
                client
                    .streaming()
                    .event_type()
                    .delete(name, Some(options.into()))
                    .await?;
            }
            Self::Patch {
                name,
                stream_event_type_patch,
            } => {
                let resp = client
                    .streaming()
                    .event_type()
                    .patch(
                        name,
                        stream_event_type_patch.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
