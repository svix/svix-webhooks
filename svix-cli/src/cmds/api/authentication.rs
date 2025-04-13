use clap::{Args, Subcommand};
use svix::api::*;

use crate::json::JsonOf;

#[derive(Args, Clone)]
pub struct AuthenticationAppPortalAccessOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationAppPortalAccessOptions>
    for svix::api::AuthenticationAppPortalAccessOptions
{
    fn from(value: AuthenticationAppPortalAccessOptions) -> Self {
        let AuthenticationAppPortalAccessOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationExpireAllOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationExpireAllOptions> for svix::api::AuthenticationExpireAllOptions {
    fn from(value: AuthenticationExpireAllOptions) -> Self {
        let AuthenticationExpireAllOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationLogoutOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationLogoutOptions> for svix::api::AuthenticationLogoutOptions {
    fn from(value: AuthenticationLogoutOptions) -> Self {
        let AuthenticationLogoutOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct AuthenticationArgs {
    #[command(subcommand)]
    pub command: AuthenticationCommands,
}

#[derive(Subcommand)]
pub enum AuthenticationCommands {
    /// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
    AppPortalAccess {
        app_id: String,
        app_portal_access_in: Option<JsonOf<AppPortalAccessIn>>,
        #[clap(flatten)]
        options: AuthenticationAppPortalAccessOptions,
    },
    /// Expire all of the tokens associated with a specific application.
    ExpireAll {
        app_id: String,
        application_token_expire_in: Option<JsonOf<ApplicationTokenExpireIn>>,
        #[clap(flatten)]
        options: AuthenticationExpireAllOptions,
    },
    /// Logout an app token.
    ///
    /// Trying to log out other tokens will fail.
    Logout {
        #[clap(flatten)]
        options: AuthenticationLogoutOptions,
    },
}

impl AuthenticationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::AppPortalAccess {
                app_id,
                app_portal_access_in,
                options,
            } => {
                let resp = client
                    .authentication()
                    .app_portal_access(
                        app_id,
                        app_portal_access_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ExpireAll {
                app_id,
                application_token_expire_in,
                options,
            } => {
                client
                    .authentication()
                    .expire_all(
                        app_id,
                        application_token_expire_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
            }
            Self::Logout { options } => {
                client.authentication().logout(Some(options.into())).await?;
            }
        }

        Ok(())
    }
}
