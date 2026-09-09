// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

use super::destination_transformation::DestinationTransformationArgs;
#[derive(Args, Clone)]
pub struct DestinationListOptions {
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

impl From<DestinationListOptions> for svix::api::DestinationListOptions {
    fn from(value: DestinationListOptions) -> Self {
        let DestinationListOptions {
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
pub struct DestinationCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<DestinationCreateOptions> for svix::api::DestinationCreateOptions {
    fn from(value: DestinationCreateOptions) -> Self {
        let DestinationCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct DestinationArgs {
    #[command(subcommand)]
    pub command: DestinationCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum DestinationCommands {
    Transformation(DestinationTransformationArgs),
    /// List of all the application's destinations.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix destination list app_abc000000000000000000000000\n",
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
        app_id: String,
        #[clap(flatten)]
        options: DestinationListOptions,
    },
    /// Creates a new destination.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix destination create app_abc000000000000000000000000 {...}\n",
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
  \"channels\": [\"...\"],
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
  \"channels\": [\"...\"],
  \"nextRetryAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Create {
        app_id: String,
        destination_in: crate::json::JsonOf<DestinationIn>,
        #[clap(flatten)]
        options: DestinationCreateOptions,
    },
    /// Get a destination by id or uid.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix destination get app_abc000000000000000000000000 DESTINATION_ID\n",
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
  \"channels\": [\"...\"],
  \"nextRetryAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Get {
        app_id: String,
        id: String,
    },
    /// Create or update a destination.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix destination upsert app_abc000000000000000000000000 DESTINATION_ID {...}\n",
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
  \"channels\": [\"...\"],
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
  \"channels\": [\"...\"],
  \"nextRetryAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Upsert {
        app_id: String,
        id: String,
        destination_in: crate::json::JsonOf<DestinationIn>,
    },
    /// Delete a destination.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix destination delete app_abc000000000000000000000000 DESTINATION_ID\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    Delete {
        app_id: String,
        id: String,
    },
    /// Partially update a destination.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix destination patch app_abc000000000000000000000000 DESTINATION_ID {...}\n",
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
  \"channels\": [\"...\"],
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
  \"channels\": [\"...\"],
  \"nextRetryAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Patch {
        app_id: String,
        id: String,
        destination_patch: crate::json::JsonOf<DestinationPatch>,
    },
}

impl DestinationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Transformation(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::List { app_id, options } => {
                let resp = client
                    .destination()
                    .list(app_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                app_id,
                destination_in,
                options,
            } => {
                let resp = client
                    .destination()
                    .create(app_id, destination_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { app_id, id } => {
                let resp = client.destination().get(app_id, id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Upsert {
                app_id,
                id,
                destination_in,
            } => {
                let resp = client
                    .destination()
                    .upsert(app_id, id, destination_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { app_id, id } => {
                client.destination().delete(app_id, id).await?;
            }
            Self::Patch {
                app_id,
                id,
                destination_patch,
            } => {
                let resp = client
                    .destination()
                    .patch(app_id, id, destination_patch.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
