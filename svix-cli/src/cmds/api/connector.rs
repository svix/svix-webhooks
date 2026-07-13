// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args, Clone)]
pub struct ConnectorListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// The sorting order of the returned items
    #[arg(long)]
    pub order: Option<Ordering>,
    #[arg(long)]
    pub product_type: Option<ConnectorProduct>,
}

impl From<ConnectorListOptions> for svix::api::ConnectorListOptions {
    fn from(value: ConnectorListOptions) -> Self {
        let ConnectorListOptions {
            limit,
            iterator,
            order,
            product_type,
        } = value;
        Self {
            limit,
            iterator,
            order,
            product_type,
        }
    }
}

#[derive(Args, Clone)]
pub struct ConnectorCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<ConnectorCreateOptions> for svix::api::ConnectorCreateOptions {
    fn from(value: ConnectorCreateOptions) -> Self {
        let ConnectorCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct ConnectorArgs {
    #[command(subcommand)]
    pub command: ConnectorCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum ConnectorCommands {
    /// List all connectors for an application.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix connector list\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"id\": \"trtmpl_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"orgId\": \"org_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"uid\": \"unique-identifier\",
    \"kind\": \"Custom\",
    \"name\": \"...\",
    \"logo\": \"...\",
    \"description\": \"...\",
    \"instructions\": \"...\",
    \"allowedEventTypes\": [\"user.signup\",\"user.deleted\"],
    \"transformation\": \"...\",
    \"createdAt\": \"2030-01-01T00:00:00Z\",
    \"updatedAt\": \"2030-01-01T00:00:00Z\",
    \"transformationUpdatedAt\": \"2030-01-01T00:00:00Z\",
    \"featureFlags\": [\"cool-new-feature\"],
    \"productType\": \"Dispatch\"
  }],
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\",
  \"done\": true
}\n")]
    List {
        #[clap(flatten)]
        options: ConnectorListOptions,
    },
    /// Create a new connector.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix connector create {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"name\": \"My first connector\",
  \"uid\": \"unique-identifier\",
  \"logo\": \"https://example.com/logo.png\",
  \"description\": \"Example connector description\",
  \"kind\": \"Slack\",
  \"instructions\": \"Markdown-formatted instructions\",
  \"allowedEventTypes\": [\"user.signup\",\"user.deleted\"],
  \"transformation\": \"function handler(webhook) { /* ... */ }\",
  \"featureFlags\": [\"...\"],
  \"productType\": \"Dispatch\"
}\n\nExample response:
{
  \"id\": \"trtmpl_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"orgId\": \"org_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"uid\": \"unique-identifier\",
  \"kind\": \"Custom\",
  \"name\": \"...\",
  \"logo\": \"...\",
  \"description\": \"...\",
  \"instructions\": \"...\",
  \"allowedEventTypes\": [\"user.signup\",\"user.deleted\"],
  \"transformation\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"transformationUpdatedAt\": \"2030-01-01T00:00:00Z\",
  \"featureFlags\": [\"cool-new-feature\"],
  \"productType\": \"Dispatch\"
}\n")]
    Create {
        connector_in: crate::json::JsonOf<ConnectorIn>,
        #[clap(flatten)]
        options: ConnectorCreateOptions,
    },
    /// Get a connector.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix connector get CONNECTOR_ID\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"id\": \"trtmpl_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"orgId\": \"org_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"uid\": \"unique-identifier\",
  \"kind\": \"Custom\",
  \"name\": \"...\",
  \"logo\": \"...\",
  \"description\": \"...\",
  \"instructions\": \"...\",
  \"allowedEventTypes\": [\"user.signup\",\"user.deleted\"],
  \"transformation\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"transformationUpdatedAt\": \"2030-01-01T00:00:00Z\",
  \"featureFlags\": [\"cool-new-feature\"],
  \"productType\": \"Dispatch\"
}\n")]
    Get { id: String },
    /// Create or update a connector.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix connector upsert CONNECTOR_ID {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"name\": \"My first connector\",
  \"logo\": \"https://example.com/logo.png\",
  \"description\": \"Example connector description\",
  \"kind\": \"Slack\",
  \"instructions\": \"Markdown-formatted instructions\",
  \"allowedEventTypes\": [\"user.signup\",\"user.deleted\"],
  \"transformation\": \"function handler(webhook) { /* ... */ }\",
  \"featureFlags\": [\"cool-new-feature\"]
}\n\nExample response:
{
  \"id\": \"trtmpl_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"orgId\": \"org_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"uid\": \"unique-identifier\",
  \"kind\": \"Custom\",
  \"name\": \"...\",
  \"logo\": \"...\",
  \"description\": \"...\",
  \"instructions\": \"...\",
  \"allowedEventTypes\": [\"user.signup\",\"user.deleted\"],
  \"transformation\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"transformationUpdatedAt\": \"2030-01-01T00:00:00Z\",
  \"featureFlags\": [\"cool-new-feature\"],
  \"productType\": \"Dispatch\"
}\n")]
    Upsert {
        id: String,
        connector_update: crate::json::JsonOf<ConnectorUpdate>,
    },
    /// Delete a connector.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix connector delete CONNECTOR_ID\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    Delete { id: String },
    /// Partially update a connector.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix connector patch CONNECTOR_ID {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"name\": \"...\",
  \"logo\": \"...\",
  \"description\": \"...\",
  \"kind\": \"Custom\",
  \"instructions\": \"...\",
  \"allowedEventTypes\": [\"user.signup\",\"user.deleted\"],
  \"transformation\": \"...\",
  \"featureFlags\": [\"cool-new-feature\"]
}\n\nExample response:
{
  \"id\": \"trtmpl_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"orgId\": \"org_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"uid\": \"unique-identifier\",
  \"kind\": \"Custom\",
  \"name\": \"...\",
  \"logo\": \"...\",
  \"description\": \"...\",
  \"instructions\": \"...\",
  \"allowedEventTypes\": [\"user.signup\",\"user.deleted\"],
  \"transformation\": \"...\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"transformationUpdatedAt\": \"2030-01-01T00:00:00Z\",
  \"featureFlags\": [\"cool-new-feature\"],
  \"productType\": \"Dispatch\"
}\n")]
    Patch {
        id: String,
        connector_patch: Option<crate::json::JsonOf<ConnectorPatch>>,
    },
}

impl ConnectorCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client.connector().list(Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                connector_in,
                options,
            } => {
                let resp = client
                    .connector()
                    .create(connector_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { id } => {
                let resp = client.connector().get(id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Upsert {
                id,
                connector_update,
            } => {
                let resp = client
                    .connector()
                    .upsert(id, connector_update.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { id } => {
                client.connector().delete(id).await?;
            }
            Self::Patch {
                id,
                connector_patch,
            } => {
                let resp = client
                    .connector()
                    .patch(id, connector_patch.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
