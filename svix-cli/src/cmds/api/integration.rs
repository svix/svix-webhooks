use clap::{Args, Subcommand};
use colored_json::ColorMode;
use svix::api::{IntegrationIn, IntegrationUpdate};

use crate::cli_types::{integration::IntegrationListOptions, PostOptions};
use crate::json::JsonOf;

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct IntegrationArgs {
    #[command(subcommand)]
    pub command: IntegrationCommands,
}

#[derive(Subcommand)]
pub enum IntegrationCommands {
    /// List the application's integrations.
    List {
        app_id: String,
        #[clap(flatten)]
        options: IntegrationListOptions,
    },
    /// Create an integration.
    Create {
        app_id: String,
        integration_in: JsonOf<IntegrationIn>,
        #[clap(flatten)]
        post_options: Option<PostOptions>,
    },
    /// Get an integration.
    Get { app_id: String, id: String },
    /// Update an integration.
    Update {
        app_id: String,
        id: String,
        integration_update: JsonOf<IntegrationUpdate>,
    },
    /// Delete an integration.
    Delete { app_id: String, id: String },
    /// Get an integration's key.
    GetKey { app_id: String, id: String },
    /// Rotate the integration's key. The previous key will be immediately revoked.
    RotateKey {
        app_id: String,
        id: String,
        // FIXME: Not part of the Rust lib (yet)
        //#[clap(flatten)]
        //post_options: Option<PostOptions>,
    },
}

impl IntegrationCommands {
    pub async fn exec(self, client: &svix::api::Svix, color_mode: ColorMode) -> anyhow::Result<()> {
        match self {
            Self::List { app_id, options } => {
                let resp = client
                    .integration()
                    .list(app_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                app_id,
                integration_in,
                post_options,
            } => {
                let resp = client
                    .integration()
                    .create(
                        app_id,
                        integration_in.into_inner(),
                        post_options.map(Into::into),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { app_id, id } => {
                let resp = client.integration().get(app_id, id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                app_id,
                id,
                integration_update,
            } => {
                let resp = client
                    .integration()
                    .update(app_id, id, integration_update.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { app_id, id } => {
                client.integration().delete(app_id, id).await?;
            }
            Self::GetKey { app_id, id } => {
                let resp = client.integration().get_key(app_id, id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::RotateKey { app_id, id } => {
                let resp = client.integration().rotate_key(app_id, id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
