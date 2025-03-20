use clap::{Args, Subcommand};
use svix::api::*;

use crate::json::JsonOf;

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

#[derive(Subcommand)]
pub enum EnvironmentCommands {
    /// Download a JSON file containing all org-settings and event types.
    Export {
        #[clap(flatten)]
        options: EnvironmentExportOptions,
    },
    /// Import a configuration into the active organization.
    ///
    /// It doesn't delete anything, only adds / updates what was passed to it.
    Import {
        environment_in: Option<JsonOf<EnvironmentIn>>,
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
