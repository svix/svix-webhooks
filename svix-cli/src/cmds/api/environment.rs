// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args, Clone)]
pub struct EnvironmentExportOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EnvironmentExportOptions> for svix::api::EnvironmentExportOptions {
    fn from(value: EnvironmentExportOptions) -> Self {
        let EnvironmentExportOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EnvironmentImportOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EnvironmentImportOptions> for svix::api::EnvironmentImportOptions {
    fn from(value: EnvironmentImportOptions) -> Self {
        let EnvironmentImportOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct EnvironmentArgs {
    #[command(subcommand)]
    pub command: EnvironmentCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum EnvironmentCommands {
    /// Download a JSON file containing all org-settings and event types.
    ///
    /// Note that the schema for [`EnvironmentOut`] is subject to change. The fields
    /// herein are provided for convenience but should be treated as JSON blobs.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix environment export\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"version\": 123,
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"eventTypes\": [{
    \"name\": \"user.signup\",
    \"description\": \"A user has signed up\",
    \"archived\": false,
    \"deprecated\": true,
    \"schemas\": {
      \"1\": {
        \"description\": \"An invoice was paid by a user\",
        \"properties\": {
          \"invoiceId\": {
            \"description\": \"The invoice id\",
            \"type\": \"string\"
          },
          \"userId\": {
            \"description\": \"The user id\",
            \"type\": \"string\"
          }
        },
        \"required\": [\"invoiceId\",\"userId\"],
        \"title\": \"Invoice Paid Event\",
        \"type\": \"object\"
      }
    },
    \"createdAt\": \"2030-01-01T00:00:00Z\",
    \"updatedAt\": \"2030-01-01T00:00:00Z\",
    \"groupName\": \"user\",
    \"featureFlags\": [\"cool-new-feature\"],
    \"featureFlag\": \"...\"
  }],
  \"settings\": {\"key\": \"...\"},
  \"connectors\": [{
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
  }]
}\n")]
    Export {
        #[clap(flatten)]
        options: EnvironmentExportOptions,
    },
    /// Import a configuration into the active organization.
    ///
    /// It doesn't delete anything, only adds / updates what was passed to it.
    ///
    /// Note that the schema for [`EnvironmentIn`] is subject to change. The fields
    /// herein are provided for convenience but should be treated as JSON blobs.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix environment import {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"eventTypes\": [{
    \"name\": \"user.signup\",
    \"description\": \"A user has signed up\",
    \"archived\": false,
    \"deprecated\": true,
    \"schemas\": {
      \"1\": {
        \"description\": \"An invoice was paid by a user\",
        \"properties\": {
          \"invoiceId\": {
            \"description\": \"The invoice id\",
            \"type\": \"string\"
          },
          \"userId\": {
            \"description\": \"The user id\",
            \"type\": \"string\"
          }
        },
        \"required\": [\"invoiceId\",\"userId\"],
        \"title\": \"Invoice Paid Event\",
        \"type\": \"object\"
      }
    },
    \"groupName\": \"user\",
    \"featureFlags\": [\"cool-new-feature\"]
  }],
  \"settings\": {\"key\": \"...\"},
  \"connectors\": [{
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
  }]
}\n")]
    Import {
        environment_in: Option<crate::json::JsonOf<EnvironmentIn>>,
        #[clap(flatten)]
        options: EnvironmentImportOptions,
    },
}

impl EnvironmentCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Export { options } => {
                let resp = client.environment().export(Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Import {
                environment_in,
                options,
            } => {
                client
                    .environment()
                    .import(
                        environment_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
