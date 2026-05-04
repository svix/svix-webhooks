// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct OperationalWebhookEndpointListOptions {
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

impl From<OperationalWebhookEndpointListOptions>
    for svix::api::OperationalWebhookEndpointListOptions
{
    fn from(value: OperationalWebhookEndpointListOptions) -> Self {
        let OperationalWebhookEndpointListOptions {
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
pub struct OperationalWebhookEndpointCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<OperationalWebhookEndpointCreateOptions>
    for svix::api::OperationalWebhookEndpointCreateOptions
{
    fn from(value: OperationalWebhookEndpointCreateOptions) -> Self {
        let OperationalWebhookEndpointCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct OperationalWebhookEndpointRotateSecretOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<OperationalWebhookEndpointRotateSecretOptions>
    for svix::api::OperationalWebhookEndpointRotateSecretOptions
{
    fn from(value: OperationalWebhookEndpointRotateSecretOptions) -> Self {
        let OperationalWebhookEndpointRotateSecretOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct OperationalWebhookEndpointArgs {
    #[command(subcommand)]
    pub command: OperationalWebhookEndpointCommands,
}

#[derive(Subcommand)]
pub enum OperationalWebhookEndpointCommands {
    /// List operational webhook endpoints.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix operational-webhook endpoint list\n",
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
    \"filterTypes\": [\"message.attempt.failing\"],
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
        #[clap(flatten)]
        options: OperationalWebhookEndpointListOptions,
    },
    /// Create an operational webhook endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix operational-webhook endpoint create {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"description\": \"An example endpoint name\",
  \"disabled\": false,
  \"filterTypes\": [\"message.attempt.failing\"],
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
  \"filterTypes\": [\"message.attempt.failing\"],
  \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"metadata\": {\"key\": \"...\"},
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"url\": \"https://example.com/webhook/\"
}\n")]
    Create {
        operational_webhook_endpoint_in: crate::json::JsonOf<OperationalWebhookEndpointIn>,
        #[clap(flatten)]
        options: OperationalWebhookEndpointCreateOptions,
    },
    /// Get an operational webhook endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix operational-webhook endpoint get ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"createdAt\": \"2030-01-01T00:00:00Z\",
  \"description\": \"...\",
  \"disabled\": false,
  \"filterTypes\": [\"message.attempt.failing\"],
  \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"metadata\": {\"key\": \"...\"},
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"url\": \"https://example.com/webhook/\"
}\n")]
    Get { endpoint_id: String },
    /// Update an operational webhook endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix operational-webhook endpoint update ep_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"description\": \"An example endpoint name\",
  \"disabled\": false,
  \"filterTypes\": [\"message.attempt.failing\"],
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
  \"filterTypes\": [\"message.attempt.failing\"],
  \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"metadata\": {\"key\": \"...\"},
  \"rateLimit\": 123,
  \"throttleRate\": 123,
  \"uid\": \"unique-identifier\",
  \"updatedAt\": \"2030-01-01T00:00:00Z\",
  \"url\": \"https://example.com/webhook/\"
}\n")]
    Update {
        endpoint_id: String,
        operational_webhook_endpoint_update: crate::json::JsonOf<OperationalWebhookEndpointUpdate>,
    },
    /// Delete an operational webhook endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix operational-webhook endpoint delete ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    Delete { endpoint_id: String },
    /// Get the additional headers to be sent with the operational webhook.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix operational-webhook endpoint get-headers ep_abc000000000000000000000000\n",
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
    GetHeaders { endpoint_id: String },
    /// Set the additional headers to be sent with the operational webhook.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix operational-webhook endpoint update-headers ep_abc000000000000000000000000 {...}\n",
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
        endpoint_id: String,
        operational_webhook_endpoint_headers_in:
            crate::json::JsonOf<OperationalWebhookEndpointHeadersIn>,
    },
    /// Get an operational webhook endpoint's signing secret.
    ///
    /// This is used to verify the authenticity of the webhook.
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix operational-webhook endpoint get-secret ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"key\": \"whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD\"
}\n")]
    GetSecret { endpoint_id: String },
    /// Rotates an operational webhook endpoint's signing secret.
    ///
    /// The previous secret will remain valid for the next 24 hours.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix operational-webhook endpoint rotate-secret ep_abc000000000000000000000000 {...}\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example body:
{
  \"key\": \"whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD\"
}\n")]
    RotateSecret {
        endpoint_id: String,
        operational_webhook_endpoint_secret_in:
            Option<crate::json::JsonOf<OperationalWebhookEndpointSecretIn>>,
        #[clap(flatten)]
        options: OperationalWebhookEndpointRotateSecretOptions,
    },
}

impl OperationalWebhookEndpointCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client
                    .operational_webhook()
                    .endpoint()
                    .list(Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                operational_webhook_endpoint_in,
                options,
            } => {
                let resp = client
                    .operational_webhook()
                    .endpoint()
                    .create(
                        operational_webhook_endpoint_in.into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { endpoint_id } => {
                let resp = client
                    .operational_webhook()
                    .endpoint()
                    .get(endpoint_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                endpoint_id,
                operational_webhook_endpoint_update,
            } => {
                let resp = client
                    .operational_webhook()
                    .endpoint()
                    .update(
                        endpoint_id,
                        operational_webhook_endpoint_update.into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { endpoint_id } => {
                client
                    .operational_webhook()
                    .endpoint()
                    .delete(endpoint_id)
                    .await?;
            }
            Self::GetHeaders { endpoint_id } => {
                let resp = client
                    .operational_webhook()
                    .endpoint()
                    .get_headers(endpoint_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::UpdateHeaders {
                endpoint_id,
                operational_webhook_endpoint_headers_in,
            } => {
                client
                    .operational_webhook()
                    .endpoint()
                    .update_headers(
                        endpoint_id,
                        operational_webhook_endpoint_headers_in.into_inner(),
                    )
                    .await?;
            }
            Self::GetSecret { endpoint_id } => {
                let resp = client
                    .operational_webhook()
                    .endpoint()
                    .get_secret(endpoint_id)
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::RotateSecret {
                endpoint_id,
                operational_webhook_endpoint_secret_in,
                options,
            } => {
                client
                    .operational_webhook()
                    .endpoint()
                    .rotate_secret(
                        endpoint_id,
                        operational_webhook_endpoint_secret_in
                            .unwrap_or_default()
                            .into_inner(),
                        Some(options.into()),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
