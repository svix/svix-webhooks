use clap::{Args, Subcommand};
use svix::api::*;

use crate::{cli_types::PostOptions, json::JsonOf};

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
    /// Expire all of the tokens associated with a specific application.
    ExpireAll {
        app_id: String,
        application_token_expire_in: JsonOf<ApplicationTokenExpireIn>,
        #[clap(flatten)]
        post_options: Option<PostOptions>,
    },
    /// Logout an app token.
    ///
    /// Trying to log out other tokens will fail.
    Logout {
        #[clap(flatten)]
        post_options: Option<PostOptions>,
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
            Self::ExpireAll {
                app_id,
                application_token_expire_in,
                post_options,
            } => {
                client
                    .authentication()
                    .expire_all(
                        app_id,
                        application_token_expire_in.into_inner(),
                        post_options.map(Into::into),
                    )
                    .await?;
            }
            Self::Logout { post_options } => {
                client
                    .authentication()
                    .logout(post_options.map(Into::into))
                    .await?;
            }
        }

        Ok(())
    }
}
