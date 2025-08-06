use clap::{Args, Subcommand};
use svix::api::*;

use super::{ingest_endpoint::IngestEndpointArgs, ingest_source::IngestSourceArgs};

#[derive(Args, Clone)]
pub struct IngestDashboardOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IngestDashboardOptions> for svix::api::IngestDashboardOptions {
    fn from(value: IngestDashboardOptions) -> Self {
        let IngestDashboardOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IngestArgs {
    #[command(subcommand)]
    pub command: IngestCommands,
}

#[derive(Subcommand)]
pub enum IngestCommands {
    Endpoint(IngestEndpointArgs),
    Source(IngestSourceArgs),
    /// Get access to the Ingest Source Consumer Portal.
    Dashboard {
        source_id: String,
        ingest_source_consumer_portal_access_in:
            Option<crate::json::JsonOf<IngestSourceConsumerPortalAccessIn>>,
        #[clap(flatten)]
        options: IngestDashboardOptions,
    },
}

impl IngestCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Endpoint(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Source(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Dashboard {
                source_id,
                ingest_source_consumer_portal_access_in,
                options,
            } => {
                let resp = client
                    .ingest()
                    .dashboard(
                        source_id,
                        ingest_source_consumer_portal_access_in
                            .unwrap_or_default()
                            .into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
