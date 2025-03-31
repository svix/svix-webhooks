use clap::{Args, Subcommand};
use svix::api::*;

use crate::json::JsonOf;

#[derive(Args, Clone)]
pub struct IngestSourceListOptions {
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

impl From<IngestSourceListOptions> for svix::api::IngestSourceListOptions {
    fn from(value: IngestSourceListOptions) -> Self {
        let IngestSourceListOptions {
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
pub struct IngestSourceCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IngestSourceCreateOptions> for svix::api::IngestSourceCreateOptions {
    fn from(value: IngestSourceCreateOptions) -> Self {
        let IngestSourceCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct IngestSourceRotateTokenOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IngestSourceRotateTokenOptions> for svix::api::IngestSourceRotateTokenOptions {
    fn from(value: IngestSourceRotateTokenOptions) -> Self {
        let IngestSourceRotateTokenOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IngestSourceArgs {
    #[command(subcommand)]
    pub command: IngestSourceCommands,
}

#[derive(Subcommand)]
pub enum IngestSourceCommands {
    /// List of all the organization's Ingest Sources.
    List {
        #[clap(flatten)]
        options: IngestSourceListOptions,
    },
    /// Create Ingest Source.
    Create {
        ingest_source_in: JsonOf<IngestSourceIn>,
        #[clap(flatten)]
        options: IngestSourceCreateOptions,
    },
    /// Get an Ingest Source by id or uid.
    Get { source_id: String },
    /// Update an Ingest Source.
    Update {
        source_id: String,
        ingest_source_in: JsonOf<IngestSourceIn>,
    },
    /// Delete an Ingest Source.
    Delete { source_id: String },
    /// Rotate the Ingest Source's Url Token.
    ///
    /// This will rotate the ingest source's token, which is used to
    /// construct the unique `ingestUrl` for the source. Previous tokens
    /// will remain valid for 48 hours after rotation. The token can be
    /// rotated a maximum of three times within the 48-hour period.
    RotateToken {
        source_id: String,
        #[clap(flatten)]
        options: IngestSourceRotateTokenOptions,
    },
}

impl IngestSourceCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client.ingest().source().list(Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                ingest_source_in,
                options,
            } => {
                let resp = client
                    .ingest()
                    .source()
                    .create(ingest_source_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { source_id } => {
                let resp = client.ingest().source().get(source_id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                source_id,
                ingest_source_in,
            } => {
                let resp = client
                    .ingest()
                    .source()
                    .update(source_id, ingest_source_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { source_id } => {
                client.ingest().source().delete(source_id).await?;
            }
            Self::RotateToken { source_id, options } => {
                let resp = client
                    .ingest()
                    .source()
                    .rotate_token(source_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
