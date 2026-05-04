// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct ApplicationListOptions {
    /// Exclude applications that have no endpoints. Default is false.
    #[arg(long)]
    pub exclude_apps_with_no_endpoints: Option<bool>,
    /// Exclude applications that have only disabled endpoints. Default is false.
    #[arg(long)]
    pub exclude_apps_with_disabled_endpoints: Option<bool>,
    /// Exclude applications that only have Svix Play endpoints. Default is false.
    #[arg(long)]
    pub exclude_apps_with_svix_play_endpoints: Option<bool>,
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

impl From<ApplicationListOptions> for svix::api::ApplicationListOptions {
    fn from(value: ApplicationListOptions) -> Self {
        let ApplicationListOptions {
            exclude_apps_with_no_endpoints,
            exclude_apps_with_disabled_endpoints,
            exclude_apps_with_svix_play_endpoints,
            limit,
            iterator,
            order,
        } = value;
        Self {
            exclude_apps_with_no_endpoints,
            exclude_apps_with_disabled_endpoints,
            exclude_apps_with_svix_play_endpoints,
            limit,
            iterator,
            order,
        }
    }
}

#[derive(Args, Clone)]
pub struct ApplicationCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<ApplicationCreateOptions> for svix::api::ApplicationCreateOptions {
    fn from(value: ApplicationCreateOptions) -> Self {
        let ApplicationCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct ApplicationArgs {
    #[command(subcommand)]
    pub command: ApplicationCommands,
}

#[derive(Subcommand)]
pub enum ApplicationCommands {
    /// List of all the organization's applications.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix application list\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"createdAt\": \"2030-01-01T00:00:00Z\",
    \"id\": \"app_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"metadata\": {\"key\": \"...\"},
    \"name\": \"My first application\",
    \"rateLimit\": 123,
    \"throttleRate\": 123,
    \"uid\": \"unique-identifier\",
    \"updatedAt\": \"2030-01-01T00:00:00Z\"
  }],
  \"done\": true,
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\"
}\n")]
    List {
        #[clap(flatten)]
        options: ApplicationListOptions,
    },
    /// Create a new application.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix application create {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"metadata\": {\"key\": \"...\"},
  \"name\": \"My first application\",
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\"
}\n\nExample response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"id\": \"app_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"metadata\": {\"key\": \"...\"},
  \"name\": \"My first application\",
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\"
}\n")]
    Create {
        application_in: crate::json::JsonOf<ApplicationIn>,
        #[clap(flatten)]
        options: ApplicationCreateOptions,
    },
    /// Get an application.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix application get app_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"id\": \"app_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"metadata\": {\"key\": \"...\"},
  \"name\": \"My first application\",
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\"
}\n")]
    Get { id: String },
    /// Update an application.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix application update app_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"metadata\": {\"key\": \"...\"},
  \"name\": \"My first application\",
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\"
}\n\nExample response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"id\": \"app_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"metadata\": {\"key\": \"...\"},
  \"name\": \"My first application\",
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\"
}\n")]
    Update {
        id: String,
        application_in: crate::json::JsonOf<ApplicationIn>,
    },
    /// Delete an application.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix application delete app_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    Delete { id: String },
    /// Partially update an application.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix application patch app_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"metadata\": {\"key\": \"...\"},
  \"name\": \"...\",
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\"
}\n\nExample response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"id\": \"app_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"metadata\": {\"key\": \"...\"},
  \"name\": \"My first application\",
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\"
}\n")]
    Patch {
        id: String,
        application_patch: Option<crate::json::JsonOf<ApplicationPatch>>,
    },
}

impl ApplicationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client.application().list(Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                application_in,
                options,
            } => {
                let resp = client
                    .application()
                    .create(application_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { id } => {
                let resp = client.application().get(id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update { id, application_in } => {
                let resp = client
                    .application()
                    .update(id, application_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { id } => {
                client.application().delete(id).await?;
            }
            Self::Patch {
                id,
                application_patch,
            } => {
                let resp = client
                    .application()
                    .patch(id, application_patch.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
