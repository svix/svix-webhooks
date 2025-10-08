// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct StreamSinkListOptions {
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

impl From<StreamSinkListOptions> for svix::api::StreamSinkListOptions {
    fn from(value: StreamSinkListOptions) -> Self {
        let StreamSinkListOptions {
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
pub struct StreamSinkCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamSinkCreateOptions> for svix::api::StreamSinkCreateOptions {
    fn from(value: StreamSinkCreateOptions) -> Self {
        let StreamSinkCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct StreamSinkRotateSecretOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamSinkRotateSecretOptions> for svix::api::StreamSinkRotateSecretOptions {
    fn from(value: StreamSinkRotateSecretOptions) -> Self {
        let StreamSinkRotateSecretOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamSinkArgs {
    #[command(subcommand)]
    pub command: StreamSinkCommands,
}

#[derive(Subcommand)]
pub enum StreamSinkCommands {
    /// List of all the stream's sinks.
    List {
        stream_id: String,
        #[clap(flatten)]
        options: StreamSinkListOptions,
    },
    /// Creates a new sink.
    Create {
        stream_id: String,
        stream_sink_in: Option<crate::json::JsonOf<StreamSinkIn>>,
        #[clap(flatten)]
        options: StreamSinkCreateOptions,
    },
    /// Get a sink by id or uid.
    Get { stream_id: String, sink_id: String },
    /// Update a sink.
    Update {
        stream_id: String,
        sink_id: String,
        stream_sink_in: Option<crate::json::JsonOf<StreamSinkIn>>,
    },
    /// Delete a sink.
    Delete { stream_id: String, sink_id: String },
    /// Partially update a sink.
    Patch {
        stream_id: String,
        sink_id: String,
        stream_sink_patch: Option<crate::json::JsonOf<StreamSinkPatch>>,
    },
    /// Get the sink's signing secret (only supported for http sinks)
    ///
    /// This is used to verify the authenticity of the delivery.
    ///
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    GetSecret { stream_id: String, sink_id: String },
    /// Rotates the signing secret (only supported for http sinks).
    RotateSecret {
        stream_id: String,
        sink_id: String,
        endpoint_secret_rotate_in: Option<crate::json::JsonOf<EndpointSecretRotateIn>>,
        #[clap(flatten)]
        options: StreamSinkRotateSecretOptions,
    },
    /// Set or unset the transformation code associated with this sink.
    TransformationPartialUpdate {
        stream_id: String,
        sink_id: String,
        sink_transform_in: Option<crate::json::JsonOf<SinkTransformIn>>,
    },
}

impl StreamSinkCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { stream_id, options } => {
                let resp = client
                    .stream()
                    .sink()
                    .list(stream_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                stream_id,
                stream_sink_in,
                options,
            } => {
                let resp = client
                    .stream()
                    .sink()
                    .create(
                        stream_id,
                        stream_sink_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { stream_id, sink_id } => {
                let resp = client.stream().sink().get(stream_id, sink_id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                stream_id,
                sink_id,
                stream_sink_in,
            } => {
                let resp = client
                    .stream()
                    .sink()
                    .update(
                        stream_id,
                        sink_id,
                        stream_sink_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { stream_id, sink_id } => {
                client.stream().sink().delete(stream_id, sink_id).await?;
            }
            Self::Patch {
                stream_id,
                sink_id,
                stream_sink_patch,
            } => {
                let resp = client
                    .stream()
                    .sink()
                    .patch(
                        stream_id,
                        sink_id,
                        stream_sink_patch.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetSecret { stream_id, sink_id } => {
                let resp = client
                    .stream()
                    .sink()
                    .get_secret(stream_id, sink_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::RotateSecret {
                stream_id,
                sink_id,
                endpoint_secret_rotate_in,
                options,
            } => {
                let resp = client
                    .stream()
                    .sink()
                    .rotate_secret(
                        stream_id,
                        sink_id,
                        endpoint_secret_rotate_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::TransformationPartialUpdate {
                stream_id,
                sink_id,
                sink_transform_in,
            } => {
                let resp = client
                    .stream()
                    .sink()
                    .transformation_partial_update(
                        stream_id,
                        sink_id,
                        sink_transform_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
