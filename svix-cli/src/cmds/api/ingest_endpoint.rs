// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

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
        source_id: String,
        #[clap(flatten)]
        options: IngestEndpointListOptions,
    },
    /// Create an ingest endpoint.
    Create {
        source_id: String,
        ingest_endpoint_in: crate::json::JsonOf<IngestEndpointIn>,
        #[clap(flatten)]
        options: IngestEndpointCreateOptions,
    },
    /// Get an ingest endpoint.
    Get {
        source_id: String,
        endpoint_id: String,
    },
    /// Update an ingest endpoint.
    Update {
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_update: crate::json::JsonOf<IngestEndpointUpdate>,
    },
    /// Delete an ingest endpoint.
    Delete {
        source_id: String,
        endpoint_id: String,
    },
    /// Get the additional headers to be sent with the ingest.
    GetHeaders {
        source_id: String,
        endpoint_id: String,
    },
    /// Set the additional headers to be sent to the endpoint.
    UpdateHeaders {
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_headers_in: crate::json::JsonOf<IngestEndpointHeadersIn>,
    },
    /// Get an ingest endpoint's signing secret.
    ///
    /// This is used to verify the authenticity of the webhook.
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    GetSecret {
        source_id: String,
        endpoint_id: String,
    },
    /// Rotates an ingest endpoint's signing secret.
    ///
    /// The previous secret will remain valid for the next 24 hours.
    RotateSecret {
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_secret_in: Option<crate::json::JsonOf<IngestEndpointSecretIn>>,
        #[clap(flatten)]
        options: IngestEndpointRotateSecretOptions,
    },
    /// Get the transformation code associated with this ingest endpoint.
    GetTransformation {
        source_id: String,
        endpoint_id: String,
    },
    /// Set or unset the transformation code associated with this ingest endpoint.
    SetTransformation {
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_transformation_patch:
            Option<crate::json::JsonOf<IngestEndpointTransformationPatch>>,
    },
}

impl IngestEndpointCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { source_id, options } => {
                let resp = client
                    .ingest()
                    .endpoint()
                    .list(source_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                source_id,
                ingest_endpoint_in,
                options,
            } => {
                let resp = client
                    .ingest()
                    .endpoint()
                    .create(
                        source_id,
                        ingest_endpoint_in.into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                source_id,
                endpoint_id,
            } => {
                let resp = client
                    .ingest()
                    .endpoint()
                    .get(source_id, endpoint_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                source_id,
                endpoint_id,
                ingest_endpoint_update,
            } => {
                let resp = client
                    .ingest()
                    .endpoint()
                    .update(source_id, endpoint_id, ingest_endpoint_update.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete {
                source_id,
                endpoint_id,
            } => {
                client
                    .ingest()
                    .endpoint()
                    .delete(source_id, endpoint_id)
                    .await?;
            }
            Self::GetHeaders {
                source_id,
                endpoint_id,
            } => {
                let resp = client
                    .ingest()
                    .endpoint()
                    .get_headers(source_id, endpoint_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::UpdateHeaders {
                source_id,
                endpoint_id,
                ingest_endpoint_headers_in,
            } => {
                client
                    .ingest()
                    .endpoint()
                    .update_headers(
                        source_id,
                        endpoint_id,
                        ingest_endpoint_headers_in.into_inner(),
                    )
                    .await?;
            }
            Self::GetSecret {
                source_id,
                endpoint_id,
            } => {
                let resp = client
                    .ingest()
                    .endpoint()
                    .get_secret(source_id, endpoint_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::RotateSecret {
                source_id,
                endpoint_id,
                ingest_endpoint_secret_in,
                options,
            } => {
                client
                    .ingest()
                    .endpoint()
                    .rotate_secret(
                        source_id,
                        endpoint_id,
                        ingest_endpoint_secret_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
            }
            Self::GetTransformation {
                source_id,
                endpoint_id,
            } => {
                let resp = client
                    .ingest()
                    .endpoint()
                    .get_transformation(source_id, endpoint_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::SetTransformation {
                source_id,
                endpoint_id,
                ingest_endpoint_transformation_patch,
            } => {
                client
                    .ingest()
                    .endpoint()
                    .set_transformation(
                        source_id,
                        endpoint_id,
                        ingest_endpoint_transformation_patch
                            .unwrap_or_default()
                            .into_inner(),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
