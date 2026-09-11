// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args, Clone)]
pub struct AutoconfigSubscriptionCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AutoconfigSubscriptionCreateOptions> for svix::api::AutoconfigSubscriptionCreateOptions {
    fn from(value: AutoconfigSubscriptionCreateOptions) -> Self {
        let AutoconfigSubscriptionCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct AutoconfigSubscriptionRotateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AutoconfigSubscriptionRotateOptions> for svix::api::AutoconfigSubscriptionRotateOptions {
    fn from(value: AutoconfigSubscriptionRotateOptions) -> Self {
        let AutoconfigSubscriptionRotateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AutoconfigSubscriptionArgs {
    #[command(subcommand)]
    pub command: AutoconfigSubscriptionCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum AutoconfigSubscriptionCommands {
    /// Create an AutoConfig subscription.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix autoconfig-subscription create app_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"token\": \"...\",
  \"id\": \"auto_1srOrx2ZWZBpBUvZwXKQmoEYga2\"
}\n")]
    Create {
        app_id: String,
        #[clap(flatten)]
        options: AutoconfigSubscriptionCreateOptions,
    },
    /// Rotate the auth token and signing secret for an AutoConfig subscription.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix autoconfig-subscription rotate app_abc000000000000000000000000 AUTOCONFIG_ID {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"signingSecret\": {
    \"key\": \"whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD\",
    \"gracePeriodSeconds\": 123
  }
}\n\nExample response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"token\": \"...\",
  \"id\": \"auto_1srOrx2ZWZBpBUvZwXKQmoEYga2\"
}\n")]
    Rotate {
        app_id: String,
        autoconfig_id: String,
        rotate_subscription_in2: Option<crate::json::JsonOf<RotateSubscriptionIn2>>,
        #[clap(flatten)]
        options: AutoconfigSubscriptionRotateOptions,
    },
}

impl AutoconfigSubscriptionCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Create { app_id, options } => {
                let resp = client
                    .autoconfig_subscription()
                    .create(app_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Rotate {
                app_id,
                autoconfig_id,
                rotate_subscription_in2,
                options,
            } => {
                let resp = client
                    .autoconfig_subscription()
                    .rotate(
                        app_id,
                        autoconfig_id,
                        rotate_subscription_in2.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
