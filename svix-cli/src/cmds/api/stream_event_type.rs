// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct StreamEventTypeListOptions {
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

impl From<StreamEventTypeListOptions> for svix::api::StreamEventTypeListOptions {
    fn from(value: StreamEventTypeListOptions) -> Self {
        let StreamEventTypeListOptions {
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
pub struct StreamEventTypeCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamEventTypeCreateOptions> for svix::api::StreamEventTypeCreateOptions {
    fn from(value: StreamEventTypeCreateOptions) -> Self {
        let StreamEventTypeCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct StreamEventTypeDeleteOptions {
    /// By default, event types are archived when "deleted". With this flag, they are deleted entirely.
    #[arg(long)]
    pub expunge: Option<bool>,
}

impl From<StreamEventTypeDeleteOptions> for svix::api::StreamEventTypeDeleteOptions {
    fn from(value: StreamEventTypeDeleteOptions) -> Self {
        let StreamEventTypeDeleteOptions { expunge } = value;
        Self { expunge }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamEventTypeArgs {
    #[command(subcommand)]
    pub command: StreamEventTypeCommands,
}

#[derive(Subcommand)]
pub enum StreamEventTypeCommands {
    /// List of all the organization's event types for streaming.
    List {
        #[clap(flatten)]
        options: StreamEventTypeListOptions,
    },
    /// Create an event type for Streams.
    Create {
        stream_event_type_in: crate::json::JsonOf<StreamEventTypeIn>,
        #[clap(flatten)]
        options: StreamEventTypeCreateOptions,
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
        options: StreamEventTypeDeleteOptions,
    },
    /// Patch an event type for Streams.
    Patch {
        name: String,
        stream_event_type_patch: Option<crate::json::JsonOf<StreamEventTypePatch>>,
    },
}

impl StreamEventTypeCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client
                    .stream()
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
                    .stream()
                    .event_type()
                    .create(stream_event_type_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { name } => {
                let resp = client.stream().event_type().get(name).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                name,
                stream_event_type_in,
            } => {
                let resp = client
                    .stream()
                    .event_type()
                    .update(name, stream_event_type_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { name, options } => {
                client
                    .stream()
                    .event_type()
                    .delete(name, Some(options.into()))
                    .await?;
            }
            Self::Patch {
                name,
                stream_event_type_patch,
            } => {
                let resp = client
                    .stream()
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
