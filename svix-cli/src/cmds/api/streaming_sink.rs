// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

use super::streaming_sink_transformation::StreamingSinkTransformationArgs;
#[derive(Args, Clone)]
pub struct StreamingSinkListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<u64>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// The sorting order of the returned items
    #[arg(long)]
    pub order: Option<Ordering>,
}

impl From<StreamingSinkListOptions> for svix::api::StreamingSinkListOptions {
    fn from(value: StreamingSinkListOptions) -> Self {
        let StreamingSinkListOptions {
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
pub struct StreamingSinkCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamingSinkCreateOptions> for svix::api::StreamingSinkCreateOptions {
    fn from(value: StreamingSinkCreateOptions) -> Self {
        let StreamingSinkCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct StreamingSinkRotateSecretOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamingSinkRotateSecretOptions> for svix::api::StreamingSinkRotateSecretOptions {
    fn from(value: StreamingSinkRotateSecretOptions) -> Self {
        let StreamingSinkRotateSecretOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamingSinkArgs {
    #[command(subcommand)]
    pub command: StreamingSinkCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum StreamingSinkCommands {
    Transformation(StreamingSinkTransformationArgs),
    /// List of all the stream's sinks.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming sink list strm_abc000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{\"...\": \"...\"}],
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\",
  \"done\": true
}\n")]
    List {
        stream_id: String,
        #[clap(flatten)]
        options: StreamingSinkListOptions,
    },
    /// Creates a new sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming sink create strm_abc000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"uid\": \"unique-identifier\",
  \"status\": \"enabled\",
  \"batchSize\": 100,
  \"maxWaitSecs\": 123,
  \"eventTypes\": [\"...\"],
  \"metadata\": {\"key\": \"...\"}
}\n\nExample response:
{
  \"id\": \"sink_2yZwUhtgs5Ai8T9yRQJXA\",
  \"uid\": \"unique-identifier\",
  \"status\": \"enabled\",
  \"currentIterator\": \"...\",
  \"failureReason\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"batchSize\": 123,
  \"maxWaitSecs\": 123,
  \"eventTypes\": [\"...\"],
  \"nextRetryAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Create {
        stream_id: String,
        stream_sink_in: crate::json::JsonOf<StreamSinkIn>,
        #[clap(flatten)]
        options: StreamingSinkCreateOptions,
    },
    /// Get a sink by id or uid.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming sink get strm_abc000000000000000000 sink_abc000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"id\": \"sink_2yZwUhtgs5Ai8T9yRQJXA\",
  \"uid\": \"unique-identifier\",
  \"status\": \"enabled\",
  \"currentIterator\": \"...\",
  \"failureReason\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"batchSize\": 123,
  \"maxWaitSecs\": 123,
  \"eventTypes\": [\"...\"],
  \"nextRetryAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Get {
        stream_id: String,
        sink_id: String,
    },
    /// Create or update a sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming sink upsert strm_abc000000000000000000 sink_abc000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"uid\": \"unique-identifier\",
  \"status\": \"enabled\",
  \"batchSize\": 100,
  \"maxWaitSecs\": 123,
  \"eventTypes\": [\"...\"],
  \"metadata\": {\"key\": \"...\"}
}\n\nExample response:
{
  \"id\": \"sink_2yZwUhtgs5Ai8T9yRQJXA\",
  \"uid\": \"unique-identifier\",
  \"status\": \"enabled\",
  \"currentIterator\": \"...\",
  \"failureReason\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"batchSize\": 123,
  \"maxWaitSecs\": 123,
  \"eventTypes\": [\"...\"],
  \"nextRetryAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Upsert {
        stream_id: String,
        sink_id: String,
        stream_sink_in: crate::json::JsonOf<StreamSinkIn>,
    },
    /// Delete a sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming sink delete strm_abc000000000000000000 sink_abc000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    Delete {
        stream_id: String,
        sink_id: String,
    },
    /// Partially update a sink.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming sink patch strm_abc000000000000000000 sink_abc000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"uid\": \"unique-identifier\",
  \"status\": \"enabled\",
  \"batchSize\": 100,
  \"maxWaitSecs\": 123,
  \"eventTypes\": [\"...\"],
  \"metadata\": {\"key\": \"...\"}
}\n\nExample response:
{
  \"id\": \"sink_2yZwUhtgs5Ai8T9yRQJXA\",
  \"uid\": \"unique-identifier\",
  \"status\": \"enabled\",
  \"currentIterator\": \"...\",
  \"failureReason\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"batchSize\": 123,
  \"maxWaitSecs\": 123,
  \"eventTypes\": [\"...\"],
  \"nextRetryAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Patch {
        stream_id: String,
        sink_id: String,
        stream_sink_patch: crate::json::JsonOf<StreamSinkPatch>,
    },
    /// Get the sink's signing secret (only supported for http sinks)
    ///
    /// This is used to verify the authenticity of the delivery.
    ///
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming sink get-secret strm_abc000000000000000000 sink_abc000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"key\": \"whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD\"
}\n")]
    GetSecret {
        stream_id: String,
        sink_id: String,
    },
    /// Rotates the signing secret (only supported for http sinks).
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming sink rotate-secret strm_abc000000000000000000 sink_abc000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"key\": \"whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD\",
  \"gracePeriodSeconds\": 123
}\n\nExample response:
{
}\n")]
    RotateSecret {
        stream_id: String,
        sink_id: String,
        endpoint_secret_rotate_in: Option<crate::json::JsonOf<EndpointSecretRotateIn>>,
        #[clap(flatten)]
        options: StreamingSinkRotateSecretOptions,
    },
}

impl StreamingSinkCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Transformation(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::List { stream_id, options } => {
                let resp = client
                    .streaming()
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
                    .streaming()
                    .sink()
                    .create(stream_id, stream_sink_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { stream_id, sink_id } => {
                let resp = client.streaming().sink().get(stream_id, sink_id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Upsert {
                stream_id,
                sink_id,
                stream_sink_in,
            } => {
                let resp = client
                    .streaming()
                    .sink()
                    .upsert(stream_id, sink_id, stream_sink_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { stream_id, sink_id } => {
                client.streaming().sink().delete(stream_id, sink_id).await?;
            }
            Self::Patch {
                stream_id,
                sink_id,
                stream_sink_patch,
            } => {
                let resp = client
                    .streaming()
                    .sink()
                    .patch(stream_id, sink_id, stream_sink_patch.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetSecret { stream_id, sink_id } => {
                let resp = client
                    .streaming()
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
                    .streaming()
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
        }

        Ok(())
    }
}
