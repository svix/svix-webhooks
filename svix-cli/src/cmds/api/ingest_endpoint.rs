// this file is @generated
use clap::{Args, Subcommand};
use svix::api::Svix;
#[allow(unused_imports)]
use svix::models::*;

use super::ingest_endpoint_transformation::IngestEndpointTransformationArgs;
#[derive(Args, Clone)]
pub struct IngestEndpointListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<u64>,

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
    Transformation(IngestEndpointTransformationArgs),
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
    \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"description\": \"...\",
    \"throttleRate\": 123,
    \"uid\": \"unique-identifier\",
    \"url\": \"https://example.com/webhook/\",
    \"disabled\": false,
    \"createdAt\": \"2030-01-01T00:00:00Z\",
    \"updatedAt\": \"2030-01-01T00:00:00Z\",
    \"metadata\": {\"key\": \"...\"}
  }],
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\",
  \"done\": true
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
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"url\": \"https://example.com/webhook/\",
  \"disabled\": false,
  \"secret\": \"whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD\",
  \"metadata\": {\"key\": \"...\"}
}\n\nExample response:
{
  \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"description\": \"...\",
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"url\": \"https://example.com/webhook/\",
  \"disabled\": false,
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
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
  \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"description\": \"...\",
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"url\": \"https://example.com/webhook/\",
  \"disabled\": false,
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Get {
        source_id: String,
        endpoint_id: String,
    },
    /// Create or update an ingest endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix ingest endpoint upsert src_abc000000000000000000 ep_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"description\": \"An example endpoint name\",
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"url\": \"https://example.com/webhook/\",
  \"disabled\": false,
  \"metadata\": {\"key\": \"...\"}
}\n\nExample response:
{
  \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"description\": \"...\",
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"url\": \"https://example.com/webhook/\",
  \"disabled\": false,
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"metadata\": {\"key\": \"...\"}
}\n")]
    Upsert {
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_upsert_in: crate::json::JsonOf<IngestEndpointUpsertIn>,
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
            "Example: svix ingest endpoint set-headers src_abc000000000000000000 ep_abc000000000000000000000000 {...}\n",
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
    SetHeaders {
        source_id: String,
        endpoint_id: String,
        ingest_endpoint_headers_in: crate::json::JsonOf<IngestEndpointHeadersIn>,
    },
}

impl IngestEndpointCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::Transformation(args) => {
                args.command.exec(client, color_mode).await?;
            }
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
            Self::Upsert {
                source_id,
                endpoint_id,
                ingest_endpoint_upsert_in,
            } => {
                let resp = client
                    .ingest()
                    .endpoint()
                    .upsert(
                        source_id,
                        endpoint_id,
                        ingest_endpoint_upsert_in.into_inner(),
                    )
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
            Self::SetHeaders {
                source_id,
                endpoint_id,
                ingest_endpoint_headers_in,
            } => {
                client
                    .ingest()
                    .endpoint()
                    .set_headers(
                        source_id,
                        endpoint_id,
                        ingest_endpoint_headers_in.into_inner(),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
