// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

use super::{
    ingest_authentication::IngestAuthenticationArgs, ingest_endpoint::IngestEndpointArgs,
    ingest_source::IngestSourceArgs,
};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IngestArgs {
    #[command(subcommand)]
    pub command: IngestCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum IngestCommands {
    Authentication(IngestAuthenticationArgs),
    Endpoint(IngestEndpointArgs),
    Source(IngestSourceArgs),
}

impl IngestCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Authentication(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Endpoint(args) => {
                args.command.exec(client, color_mode).await?;
            }
            Self::Source(args) => {
                args.command.exec(client, color_mode).await?;
            }
        }

        Ok(())
    }
}
