use clap::{Args, Subcommand};
use colored_json::ColorMode;
use svix::api::AppPortalAccessIn;

use crate::{cli_types::PostOptions, config::Config, get_client_options, json::JsonOf};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct AuthenticationArgs {
    #[command(subcommand)]
    pub command: AuthenticationCommands,
}

#[derive(Subcommand)]
pub enum AuthenticationCommands {
    /// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
    AppPortalAccess {
        app_id: String,
        app_portal_access_in: JsonOf<AppPortalAccessIn>,
        #[clap(flatten)]
        post_options: Option<PostOptions>,
    },
    /// Logout an app token.
    ///
    /// Trying to log out other tokens will fail.
    Logout {
        dashboard_auth_token: String,
        #[clap(flatten)]
        post_options: Option<PostOptions>,
    },
}

impl AuthenticationCommands {
    pub async fn exec(
        self,
        client: &svix::api::Svix,
        color_mode: ColorMode,
        cfg: &Config,
    ) -> anyhow::Result<()> {
        match self {
            Self::AppPortalAccess {
                app_id,
                app_portal_access_in,
                post_options,
            } => {
                let resp = client
                    .authentication()
                    .app_portal_access(
                        app_id,
                        app_portal_access_in.into_inner(),
                        post_options.map(Into::into),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            AuthenticationCommands::Logout {
                dashboard_auth_token,
                post_options,
            } => {
                // We're not using the client received by `exec()` here since the token is an
                // arg, not whatever is configured for the cli otherwise.
                let client =
                    svix::api::Svix::new(dashboard_auth_token, Some(get_client_options(cfg)?));

                client
                    .authentication()
                    .logout(post_options.map(Into::into))
                    .await?;
            }
        }

        Ok(())
    }
}
