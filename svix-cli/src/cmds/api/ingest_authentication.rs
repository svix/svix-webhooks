// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

#[derive(Args, Clone)]
pub struct IngestAuthenticationConsumerPortalAccessOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IngestAuthenticationConsumerPortalAccessOptions>
    for svix::api::IngestAuthenticationConsumerPortalAccessOptions
{
    fn from(value: IngestAuthenticationConsumerPortalAccessOptions) -> Self {
        let IngestAuthenticationConsumerPortalAccessOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IngestAuthenticationArgs {
    #[command(subcommand)]
    pub command: IngestAuthenticationCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum IngestAuthenticationCommands {
    /// Get access to the Ingest Source Consumer Portal.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest authentication consumer-portal-access src_abc000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"expiry\": 123,
  \"readOnly\": true
}\n\nExample response:
{
  \"url\": \"https://app.svix.com/login#key=eyJhcHBJZCI6ICJhcHBfMXRSdFl\",
  \"token\": \"appsk_kV3ts5tKPNJN4Dl25cMTfUNdmabxbX0O\"
}\n")]
    ConsumerPortalAccess {
        source_id: String,
        ingest_source_consumer_portal_access_in:
            Option<crate::json::JsonOf<IngestSourceConsumerPortalAccessIn>>,
        #[clap(flatten)]
        options: IngestAuthenticationConsumerPortalAccessOptions,
    },
}

impl IngestAuthenticationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::ConsumerPortalAccess {
                source_id,
                ingest_source_consumer_portal_access_in,
                options,
            } => {
                let resp = client
                    .ingest()
                    .authentication()
                    .consumer_portal_access(
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
