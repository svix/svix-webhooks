use clap::{Args, Subcommand};
use colored_json::ColorMode;
use svix::api::{ApplicationIn, ApplicationPatch};

use crate::{
    cli_types::{application::ApplicationListOptions, PostOptions},
    json::JsonOf,
};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct ApplicationArgs {
    #[command(subcommand)]
    pub command: ApplicationCommands,
}

#[derive(Subcommand)]
pub enum ApplicationCommands {
    /// List of all the organization's applications.
    List {
        #[clap(flatten)]
        options: ApplicationListOptions,
    },
    /// Create a new application.
    Create {
        application_in: JsonOf<ApplicationIn>,
        #[clap(flatten)]
        post_options: Option<PostOptions>,
    },
    /// Get an application.
    Get { id: String },
    /// Update an application.
    Update {
        id: String,
        application_in: JsonOf<ApplicationIn>,
    },
    /// Delete an application.
    Delete { id: String },
    /// Partially update an application.
    Patch {
        id: String,
        application_patch: JsonOf<ApplicationPatch>,
    },
}

impl ApplicationCommands {
    pub async fn exec(self, client: &svix::api::Svix, color_mode: ColorMode) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client.application().list(Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                application_in,
                post_options,
            } => {
                let resp = client
                    .application()
                    .create(application_in.into_inner(), post_options.map(Into::into))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { id } => {
                let resp = client.application().get(id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update { id, application_in } => {
                let resp = client
                    .application()
                    .update(id, application_in.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { id } => {
                client.application().delete(id).await?;
            }
            Self::Patch {
                id,
                application_patch,
            } => {
                let resp = client
                    .application()
                    .patch(id, application_patch.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
