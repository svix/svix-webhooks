// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

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

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum IngestEndpointCommands {
    /// List ingest endpoints.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint list src_abc000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"createdAt\": \"2030-01-01T00:00:00Z\",
    \"description\": \"...\",
    \"disabled\": false,
    \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"metadata\": {\"key\": \"...\"},
    \"rateLimit\": 123,
    \"throttleRate\": 123,
    \"uid\": \"unique-identifier\",
    \"updatedAt\": \"2030-01-01T00:00:00Z\",
    \"url\": \"https://example.com/webhook/\"
  }],
  \"done\": true,
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\"
}\n")]
    List {
        source_id: String,
        #[clap(flatten)]
        options: IngestEndpointListOptions,
    },
    /// Create an ingest endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint create src_abc000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"description\": \"An example endpoint name\",
  \"disabled\": false,
  \"metadata\": {\"key\": \"...\"},
  \"rateLimit\": 123,
  \"secret\": \"whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD\",
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"url\": \"https://example.com/webhook/\"
}\n\nExample response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"description\": \"...\",
  \"disabled\": false,
  \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"metadata\": {\"key\": \"...\"},
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"url\": \"https://example.com/webhook/\"
}\n")]
    Create {
        source_id: String,
        ingest_endpoint_in: crate::json::JsonOf<IngestEndpointIn>,
        #[clap(flatten)]
        options: IngestEndpointCreateOptions,
    },
    /// Get an ingest endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint get src_abc000000000000000000 ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"description\": \"...\",
  \"disabled\": false,
  \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"metadata\": {\"key\": \"...\"},
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"url\": \"https://example.com/webhook/\"
}\n")]
    Get {
        source_id: String,
        endpoint_id: String,
    },
    /// Create or update an ingest endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint update src_abc000000000000000000 ep_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"description\": \"An example endpoint name\",
  \"disabled\": false,
  \"metadata\": {\"key\": \"...\"},
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"url\": \"https://example.com/webhook/\"
}\n\nExample response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"description\": \"...\",
  \"disabled\": false,
  \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"metadata\": {\"key\": \"...\"},
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"url\": \"https://example.com/webhook/\"
}\n")]
    Update {
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_update: crate::json::JsonOf<IngestEndpointUpdate>,
    },
    /// Delete an ingest endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint delete src_abc000000000000000000 ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    Delete {
        source_id: String,
        endpoint_id: String,
    },
    /// Get an ingest endpoint's signing secret.
    ///
    /// This is used to verify the authenticity of the webhook.
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint get-secret src_abc000000000000000000 ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"key\": \"whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD\"
}\n")]
    GetSecret {
        source_id: String,
        endpoint_id: String,
    },
    /// Rotates an ingest endpoint's signing secret.
    ///
    /// The previous secret will remain valid for the next 24 hours.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint rotate-secret src_abc000000000000000000 ep_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"key\": \"whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD\"
}\n")]
    RotateSecret {
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_secret_in: Option<crate::json::JsonOf<IngestEndpointSecretIn>>,
        #[clap(flatten)]
        options: IngestEndpointRotateSecretOptions,
    },
    /// Get the additional headers to be sent with the ingest.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint get-headers src_abc000000000000000000 ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"headers\": {
    \"X-Example\": \"123\",
    \"X-Foobar\": \"Bar\"
  },
  \"sensitive\": [\"Authorization\"]
}\n")]
    GetHeaders {
        source_id: String,
        endpoint_id: String,
    },
    /// Set the additional headers to be sent to the endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint update-headers src_abc000000000000000000 ep_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"headers\": {
    \"X-Example\": \"123\",
    \"X-Foobar\": \"Bar\"
  }
}\n")]
    UpdateHeaders {
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_headers_in: crate::json::JsonOf<IngestEndpointHeadersIn>,
    },
    /// Get the transformation code associated with this ingest endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint get-transformation src_abc000000000000000000 ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"code\": \"...\",
  \"enabled\": true
}\n")]
    GetTransformation {
        source_id: String,
        endpoint_id: String,
    },
    /// Set or unset the transformation code associated with this ingest endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint set-transformation src_abc000000000000000000 ep_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"code\": \"...\",
  \"enabled\": true
}\n")]
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
