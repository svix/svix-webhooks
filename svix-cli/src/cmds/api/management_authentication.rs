use clap::{Args, Subcommand};
use svix::api::*;

use crate::json::JsonOf;

#[derive(Args, Clone)]
pub struct ManagementAuthenticationListApiTokensOptions {
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

impl From<ManagementAuthenticationListApiTokensOptions>
    for svix::api::ManagementAuthenticationListApiTokensOptions
{
    fn from(value: ManagementAuthenticationListApiTokensOptions) -> Self {
        let ManagementAuthenticationListApiTokensOptions {
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
pub struct ManagementAuthenticationCreateApiTokenOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<ManagementAuthenticationCreateApiTokenOptions>
    for svix::api::ManagementAuthenticationCreateApiTokenOptions
{
    fn from(value: ManagementAuthenticationCreateApiTokenOptions) -> Self {
        let ManagementAuthenticationCreateApiTokenOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct ManagementAuthenticationExpireApiTokenOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<ManagementAuthenticationExpireApiTokenOptions>
    for svix::api::ManagementAuthenticationExpireApiTokenOptions
{
    fn from(value: ManagementAuthenticationExpireApiTokenOptions) -> Self {
        let ManagementAuthenticationExpireApiTokenOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct ManagementAuthenticationArgs {
    #[command(subcommand)]
    pub command: ManagementAuthenticationCommands,
}

#[derive(Subcommand)]
pub enum ManagementAuthenticationCommands {
    /// List all API Tokens.
    ListApiTokens {
        #[clap(flatten)]
        options: ManagementAuthenticationListApiTokensOptions,
    },
    /// Create a new API Token.
    CreateApiToken {
        api_token_in: JsonOf<ApiTokenIn>,
        #[clap(flatten)]
        options: ManagementAuthenticationCreateApiTokenOptions,
    },
    /// Expire the selected API Token.
    ExpireApiToken {
        key_id: String,
        api_token_expire_in: Option<JsonOf<ApiTokenExpireIn>>,
        #[clap(flatten)]
        options: ManagementAuthenticationExpireApiTokenOptions,
    },
}

impl ManagementAuthenticationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::ListApiTokens { options } => {
                let resp = client
                    .management()
                    .authentication()
                    .list_api_tokens(Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::CreateApiToken {
                api_token_in,
                options,
            } => {
                let resp = client
                    .management()
                    .authentication()
                    .create_api_token(api_token_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ExpireApiToken {
                key_id,
                api_token_expire_in,
                options,
            } => {
                client
                    .management()
                    .authentication()
                    .expire_api_token(
                        key_id,
                        api_token_expire_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
