use clap::{Args, Subcommand};
use svix::api::{self, Ordering};

use crate::{cli_types::PostOptions, json::JsonOf};

#[derive(Args, Clone)]
pub struct ApplicationListOptions {
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

impl From<ApplicationListOptions> for api::ApplicationListOptions {
    fn from(
        ApplicationListOptions {
            limit,
            iterator,
            order,
        }: ApplicationListOptions,
    ) -> Self {
        Self {
            limit,
            iterator,
            order,
        }
    }
}

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
        application_in: JsonOf<api::ApplicationIn>,
        #[clap(flatten)]
        post_options: Option<PostOptions>,
    },
    /// Get an application.
    Get { id: String },
    /// Update an application.
    Update {
        id: String,
        application_in: JsonOf<api::ApplicationIn>,
    },
    /// Delete an application.
    Delete { id: String },
    /// Partially update an application.
    Patch {
        id: String,
        application_patch: JsonOf<api::ApplicationPatch>,
    },
}

impl ApplicationCommands {
    pub async fn exec(
        self,
        client: &api::Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
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
