// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args, Clone)]
pub struct IntegrationListOptions {
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

impl From<IntegrationListOptions> for svix::api::IntegrationListOptions {
    fn from(value: IntegrationListOptions) -> Self {
        let IntegrationListOptions {
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
pub struct IntegrationCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IntegrationCreateOptions> for svix::api::IntegrationCreateOptions {
    fn from(value: IntegrationCreateOptions) -> Self {
        let IntegrationCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct IntegrationRotateKeyOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IntegrationRotateKeyOptions> for svix::api::IntegrationRotateKeyOptions {
    fn from(value: IntegrationRotateKeyOptions) -> Self {
        let IntegrationRotateKeyOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IntegrationArgs {
    #[command(subcommand)]
    pub command: IntegrationCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum IntegrationCommands {
    /// List the application's integrations.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix integration list app_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"name\": \"Example Integration\",
    \"id\": \"integ_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"createdAt\": \"2030-01-01T00:00:00Z\",
    \"updatedAt\": \"2030-01-01T00:00:00Z\",
    \"featureFlags\": []
  }],
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\",
  \"done\": true
}\n")]
    List {
        app_id: String,
        #[clap(flatten)]
        options: IntegrationListOptions,
    },
    /// Create an integration.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix integration create app_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"name\": \"Example Integration\",
  \"featureFlags\": []
}\n\nExample response:
{
  \"name\": \"Example Integration\",
  \"id\": \"integ_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"featureFlags\": []
}\n")]
    Create {
        app_id: String,
        integration_in: crate::json::JsonOf<IntegrationIn>,
        #[clap(flatten)]
        options: IntegrationCreateOptions,
    },
    /// Get an integration.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix integration get app_abc000000000000000000000000 integ_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"name\": \"Example Integration\",
  \"id\": \"integ_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"featureFlags\": []
}\n")]
    Get { app_id: String, id: String },
    /// Update an integration.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix integration update app_abc000000000000000000000000 integ_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"name\": \"Example Integration\",
  \"featureFlags\": []
}\n\nExample response:
{
  \"name\": \"Example Integration\",
  \"id\": \"integ_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"featureFlags\": []
}\n")]
    Update {
        app_id: String,
        id: String,
        integration_update: crate::json::JsonOf<IntegrationUpdate>,
    },
    /// Delete an integration.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix integration delete app_abc000000000000000000000000 integ_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    Delete { app_id: String, id: String },
    /// Rotate the integration's key. The previous key will be immediately revoked.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix integration rotate-key app_abc000000000000000000000000 integ_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"key\": \"integsk_kV3ts5tKPNJN4Dl25cMTfUNdmabxbX0O\"
}\n")]
    RotateKey {
        app_id: String,
        id: String,
        #[clap(flatten)]
        options: IntegrationRotateKeyOptions,
    },
}

impl IntegrationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { app_id, options } => {
                let resp = client
                    .integration()
                    .list(app_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                app_id,
                integration_in,
                options,
            } => {
                let resp = client
                    .integration()
                    .create(app_id, integration_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { app_id, id } => {
                let resp = client.integration().get(app_id, id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                app_id,
                id,
                integration_update,
            } => {
                let resp = client
                    .integration()
                    .update(app_id, id, integration_update.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { app_id, id } => {
                client.integration().delete(app_id, id).await?;
            }
            Self::RotateKey {
                app_id,
                id,
                options,
            } => {
                let resp = client
                    .integration()
                    .rotate_key(app_id, id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
