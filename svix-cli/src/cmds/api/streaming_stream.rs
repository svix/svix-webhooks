// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args, Clone)]
pub struct StreamingStreamListOptions {
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

impl From<StreamingStreamListOptions> for svix::api::StreamingStreamListOptions {
    fn from(value: StreamingStreamListOptions) -> Self {
        let StreamingStreamListOptions {
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
pub struct StreamingStreamCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<StreamingStreamCreateOptions> for svix::api::StreamingStreamCreateOptions {
    fn from(value: StreamingStreamCreateOptions) -> Self {
        let StreamingStreamCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct StreamingStreamArgs {
    #[command(subcommand)]
    pub command: StreamingStreamCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum StreamingStreamCommands {
    /// List of all the organization's streams.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming stream list\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"id\": \"strm_2yZwUhtgs5Ai8T9yRQJXA\",
    \"uid\": \"unique-identifier\",
    \"name\": \"...\",
    \"createdAt\": \"2030-01-01T00:00:00Z\",
    \"updatedAt\": \"2030-01-01T00:00:00Z\",
    \"metadata\": {\"key\": \"...\"}
  }],
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\",
  \"done\": true
}\n")]
    List {
        #[clap(flatten)]
        options: StreamingStreamListOptions,
    },
    /// Creates a new stream.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming stream create {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"name\": \"...\",
  \"uid\": \"unique-identifier\",
  \"metadata\": {\"key\": \"...\"}
}\n\nExample response:
{
  \"id\": \"strm_2yZwUhtgs5Ai8T9yRQJXA\",
  \"uid\": \"unique-identifier\",
  \"name\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Create {
        stream_in: crate::json::JsonOf<StreamIn>,
        #[clap(flatten)]
        options: StreamingStreamCreateOptions,
    },
    /// Get a stream by id or uid.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming stream get strm_abc000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"id\": \"strm_2yZwUhtgs5Ai8T9yRQJXA\",
  \"uid\": \"unique-identifier\",
  \"name\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Get { stream_id: String },
    /// Create or update a stream.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming stream upsert strm_abc000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"name\": \"...\",
  \"uid\": \"unique-identifier\",
  \"metadata\": {\"key\": \"...\"}
}\n\nExample response:
{
  \"id\": \"strm_2yZwUhtgs5Ai8T9yRQJXA\",
  \"uid\": \"unique-identifier\",
  \"name\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Upsert {
        stream_id: String,
        stream_in: crate::json::JsonOf<StreamIn>,
    },
    /// Delete a stream.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming stream delete strm_abc000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    Delete { stream_id: String },
    /// Partially update a stream.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix streaming stream patch strm_abc000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"description\": \"...\",
  \"uid\": \"unique-identifier\",
  \"metadata\": {\"key\": \"...\"}
}\n\nExample response:
{
  \"id\": \"strm_2yZwUhtgs5Ai8T9yRQJXA\",
  \"uid\": \"unique-identifier\",
  \"name\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Patch {
        stream_id: String,
        stream_patch: Option<crate::json::JsonOf<StreamPatch>>,
    },
}

impl StreamingStreamCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client
                    .streaming()
                    .stream()
                    .list(Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create { stream_in, options } => {
                let resp = client
                    .streaming()
                    .stream()
                    .create(stream_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { stream_id } => {
                let resp = client.streaming().stream().get(stream_id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Upsert {
                stream_id,
                stream_in,
            } => {
                let resp = client
                    .streaming()
                    .stream()
                    .upsert(stream_id, stream_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { stream_id } => {
                client.streaming().stream().delete(stream_id).await?;
            }
            Self::Patch {
                stream_id,
                stream_patch,
            } => {
                let resp = client
                    .streaming()
                    .stream()
                    .patch(stream_id, stream_patch.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
