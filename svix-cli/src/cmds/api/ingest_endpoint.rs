use chrono::{DateTime, Utc};
use clap::{Args, Subcommand};
use svix::api::*;

use crate::json::JsonOf;

#[derive(Args, Clone)]
pub struct IngestEndpointListOptions {
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

impl From<IngestEndpointListOptions> for svix::api::IngestEndpointListOptions {
    fn from(value: IngestEndpointListOptions) -> Self {
        let IngestEndpointListOptions {
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
pub struct IngestEndpointCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IngestEndpointCreateOptions> for svix::api::IngestEndpointCreateOptions {
    fn from(value: IngestEndpointCreateOptions) -> Self {
        let IngestEndpointCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct IngestEndpointRotateSecretOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IngestEndpointRotateSecretOptions> for svix::api::IngestEndpointRotateSecretOptions {
    fn from(value: IngestEndpointRotateSecretOptions) -> Self {
        let IngestEndpointRotateSecretOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct IngestEndpointArgs {
    #[command(subcommand)]
    pub command: IngestEndpointCommands,
}

#[derive(Subcommand)]
pub enum IngestEndpointCommands {
    /// List ingest endpoints.
    List {
        #[clap(flatten)]
        options: IngestEndpointListOptions,
    },
    /// Create an ingest endpoint.
    Create {
        ingest_endpoint_in: JsonOf<IngestEndpointIn>,
        #[clap(flatten)]
        options: IngestEndpointCreateOptions,
    },
    /// Get an ingest endpoint.
    Get { endpoint_id: String },
    /// Update an ingest endpoint.
    Update {
        endpoint_id: String,
        ingest_endpoint_update: JsonOf<IngestEndpointUpdate>,
    },
    /// Delete an ingest endpoint.
    Delete { endpoint_id: String },
    /// Get the additional headers to be sent with the ingest.
    GetHeaders { endpoint_id: String },
    /// Set the additional headers to be sent to the endpoint.
    UpdateHeaders {
        endpoint_id: String,
        ingest_endpoint_headers_in: JsonOf<IngestEndpointHeadersIn>,
    },
    /// Get an ingest endpoint's signing secret.
    ///
    /// This is used to verify the authenticity of the webhook.
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    GetSecret { endpoint_id: String },
    /// Rotates an ingest endpoint's signing secret.
    ///
    /// The previous secret will remain valid for the next 24 hours.
    RotateSecret {
        endpoint_id: String,
        ingest_endpoint_secret_in: Option<JsonOf<IngestEndpointSecretIn>>,
        #[clap(flatten)]
        options: IngestEndpointRotateSecretOptions,
    },
}

impl IngestEndpointCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client.ingest_endpoint().list(Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                ingest_endpoint_in,
                options,
            } => {
                let resp = client
                    .ingest_endpoint()
                    .create(ingest_endpoint_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { endpoint_id } => {
                let resp = client.ingest_endpoint().get(endpoint_id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                endpoint_id,
                ingest_endpoint_update,
            } => {
                let resp = client
                    .ingest_endpoint()
                    .update(endpoint_id, ingest_endpoint_update.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { endpoint_id } => {
                client.ingest_endpoint().delete(endpoint_id).await?;
            }
            Self::GetHeaders { endpoint_id } => {
                let resp = client.ingest_endpoint().get_headers(endpoint_id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::UpdateHeaders {
                endpoint_id,
                ingest_endpoint_headers_in,
            } => {
                client
                    .ingest_endpoint()
                    .update_headers(endpoint_id, ingest_endpoint_headers_in.into_inner())
                    .await?;
            }
            Self::GetSecret { endpoint_id } => {
                let resp = client.ingest_endpoint().get_secret(endpoint_id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::RotateSecret {
                endpoint_id,
                ingest_endpoint_secret_in,
                options,
            } => {
                client
                    .ingest_endpoint()
                    .rotate_secret(
                        endpoint_id,
                        ingest_endpoint_secret_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
