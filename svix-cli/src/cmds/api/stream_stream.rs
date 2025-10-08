// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct StreamStreamListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// The sorting order of the returned items
    #[arg(long)]
    pub order: Option<Ordering>,
}

impl From<StreamStreamListOptions> for svix::api::StreamStreamListOptions {
    fn from(value: StreamStreamListOptions) -> Self {
        let StreamStreamListOptions {
            limit,
            iterator,
            order,
        } = value;
        Self {
            limit,
            iterator,
            order,
        }
    }
}

#[derive(Args, Clone)]
pub struct StreamStreamCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamStreamCreateOptions> for svix::api::StreamStreamCreateOptions {
    fn from(value: StreamStreamCreateOptions) -> Self {
        let StreamStreamCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamStreamArgs {
    #[command(subcommand)]
    pub command: StreamStreamCommands,
}

#[derive(Subcommand)]
pub enum StreamStreamCommands {
    /// List of all the organization's streams.
    List {
        #[clap(flatten)]
        options: StreamStreamListOptions,
    },
    /// Creates a new stream.
    Create {
        stream_in: crate::json::JsonOf<StreamIn>,
        #[clap(flatten)]
        options: StreamStreamCreateOptions,
    },
    /// Get a stream by id or uid.
    Get { stream_id: String },
    /// Update a stream.
    Update {
        stream_id: String,
        stream_in: crate::json::JsonOf<StreamIn>,
    },
    /// Delete a stream.
    Delete { stream_id: String },
    /// Partially update a stream.
    Patch {
        stream_id: String,
        stream_patch: Option<crate::json::JsonOf<StreamPatch>>,
    },
}

impl StreamStreamCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client.stream().stream().list(Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create { stream_in, options } => {
                let resp = client
                    .stream()
                    .stream()
                    .create(stream_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { stream_id } => {
                let resp = client.stream().stream().get(stream_id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                stream_id,
                stream_in,
            } => {
                let resp = client
                    .stream()
                    .stream()
                    .update(stream_id, stream_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { stream_id } => {
                client.stream().stream().delete(stream_id).await?;
            }
            Self::Patch {
                stream_id,
                stream_patch,
            } => {
                let resp = client
                    .stream()
                    .stream()
                    .patch(stream_id, stream_patch.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
