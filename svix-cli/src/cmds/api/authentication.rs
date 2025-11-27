// this file is @generated
use clap::{
    Args,
    Subcommand,
};
use svix::api::*;

#[derive(Args, Clone)]
pub struct AuthenticationAppPortalAccessOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationAppPortalAccessOptions>
    for svix::api::AuthenticationAppPortalAccessOptions
{
    fn from(value: AuthenticationAppPortalAccessOptions) -> Self {
        let AuthenticationAppPortalAccessOptions {
            idempotency_key,
        } = value;
        Self {
            idempotency_key,
        }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationExpireAllOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationExpireAllOptions> for svix::api::AuthenticationExpireAllOptions {
    fn from(value: AuthenticationExpireAllOptions) -> Self {
        let AuthenticationExpireAllOptions {
            idempotency_key,
        } = value;
        Self {
            idempotency_key,
        }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationLogoutOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationLogoutOptions> for svix::api::AuthenticationLogoutOptions {
    fn from(value: AuthenticationLogoutOptions) -> Self {
        let AuthenticationLogoutOptions {
            idempotency_key,
        } = value;
        Self {
            idempotency_key,
        }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationStreamPortalAccessOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationStreamPortalAccessOptions>
    for svix::api::AuthenticationStreamPortalAccessOptions
{
    fn from(value: AuthenticationStreamPortalAccessOptions) -> Self {
        let AuthenticationStreamPortalAccessOptions {
            idempotency_key,
        } = value;
        Self {
            idempotency_key,
        }
    }
}

#[derive(Args, Clone)]
pub struct AuthenticationRotateStreamPollerTokenOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<AuthenticationRotateStreamPollerTokenOptions>
    for svix::api::AuthenticationRotateStreamPollerTokenOptions
{
    fn from(value: AuthenticationRotateStreamPollerTokenOptions) -> Self {
        let AuthenticationRotateStreamPollerTokenOptions {
            idempotency_key,
        } = value;
        Self {
            idempotency_key,
        }
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
        app_portal_access_in: Option<crate::json::JsonOf<AppPortalAccessIn>>,
        #[clap(flatten)]
        options: AuthenticationAppPortalAccessOptions,
    },
    /// Expire all of the tokens associated with a specific application.
    ExpireAll {
        app_id: String,
        application_token_expire_in: Option<crate::json::JsonOf<ApplicationTokenExpireIn>>,
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
    /// Use this function to get magic links (and authentication codes) for connecting your users to the Stream Consumer Portal.
    StreamPortalAccess {
        stream_id: String,
        stream_portal_access_in: Option<crate::json::JsonOf<StreamPortalAccessIn>>,
        #[clap(flatten)]
        options: AuthenticationStreamPortalAccessOptions,
    },
    /// Get the current auth token for the stream poller.
    GetStreamPollerToken { stream_id: String, sink_id: String },
    /// Create a new auth token for the stream poller API.
    RotateStreamPollerToken {
        stream_id: String,
        sink_id: String,
        rotate_poller_token_in: Option<crate::json::JsonOf<RotatePollerTokenIn>>,
        #[clap(flatten)]
        options: AuthenticationRotateStreamPollerTokenOptions,
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
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
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
            Self::Logout {
                options,
            } => {
                client.authentication().logout(Some(options.into())).await?;
            }
            Self::StreamPortalAccess {
                stream_id,
                stream_portal_access_in,
                options,
            } => {
                let resp = client
                    .authentication()
                    .stream_portal_access(
                        stream_id,
                        stream_portal_access_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::GetStreamPollerToken {
                stream_id,
                sink_id,
            } => {
                let resp = client
                    .authentication()
                    .get_stream_poller_token(
                        stream_id, sink_id,
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::RotateStreamPollerToken {
                stream_id,
                sink_id,
                rotate_poller_token_in,
                options,
            } => {
                let resp = client
                    .authentication()
                    .rotate_stream_poller_token(
                        stream_id,
                        sink_id,
                        rotate_poller_token_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
        }

        Ok(())
    }
}
