// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

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
  \"connectors\": [{
    \"allowedEventTypes\": [\"user.signup\",\"user.deleted\"],
    \"createdAt\": \"2030-01-01T00:00:00Z\",
    \"description\": \"...\",
    \"featureFlags\": [\"cool-new-feature\"],
    \"id\": \"trtmpl_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"instructions\": \"...\",
    \"kind\": \"Custom\",
    \"logo\": \"...\",
    \"name\": \"...\",
    \"orgId\": \"org_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"productType\": \"Dispatch\",
    \"transformation\": \"...\",
    \"transformationUpdatedAt\": \"2030-01-01T00:00:00Z\",
    \"uid\": \"unique-identifier\",
    \"updatedAt\": \"2030-01-01T00:00:00Z\"
  }],
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"eventTypes\": [{
    \"archived\": false,
    \"createdAt\": \"2030-01-01T00:00:00Z\",
    \"deprecated\": true,
    \"description\": \"A user has signed up\",
    \"featureFlag\": \"...\",
    \"featureFlags\": [\"cool-new-feature\"],
    \"groupName\": \"user\",
    \"name\": \"user.signup\",
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
    \"updatedAt\": \"2030-01-01T00:00:00Z\"
  }],
  \"settings\": {\"key\": \"...\"},
  \"version\": 123
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
  \"connectors\": [{
    \"allowedEventTypes\": [\"user.signup\",\"user.deleted\"],
    \"description\": \"Example connector description\",
    \"featureFlags\": [\"...\"],
    \"instructions\": \"Markdown-formatted instructions\",
    \"kind\": \"Slack\",
    \"logo\": \"https://example.com/logo.png\",
    \"name\": \"My first connector\",
    \"productType\": \"Dispatch\",
    \"transformation\": \"function handler(webhook) { /* ... */ }\",
    \"uid\": \"unique-identifier\"
  }],
  \"eventTypes\": [{
    \"archived\": false,
    \"deprecated\": true,
    \"description\": \"A user has signed up\",
    \"featureFlag\": \"...\",
    \"featureFlags\": [\"cool-new-feature\"],
    \"groupName\": \"user\",
    \"name\": \"user.signup\",
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
    }
  }],
  \"settings\": {\"key\": \"...\"}
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
