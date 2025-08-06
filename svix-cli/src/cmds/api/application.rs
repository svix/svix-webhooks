use clap::{Args, Subcommand};
use svix::api::*;

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

impl From<ApplicationListOptions> for svix::api::ApplicationListOptions {
    fn from(value: ApplicationListOptions) -> Self {
        let ApplicationListOptions {
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
pub struct ApplicationCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<ApplicationCreateOptions> for svix::api::ApplicationCreateOptions {
    fn from(value: ApplicationCreateOptions) -> Self {
        let ApplicationCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
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
        application_in: crate::json::JsonOf<ApplicationIn>,
        #[clap(flatten)]
        options: ApplicationCreateOptions,
    },
    /// Get an application.
    Get { id: String },
    /// Update an application.
    Update {
        id: String,
        application_in: crate::json::JsonOf<ApplicationIn>,
    },
    /// Delete an application.
    Delete { id: String },
    /// Partially update an application.
    Patch {
        id: String,
        application_patch: Option<crate::json::JsonOf<ApplicationPatch>>,
    },
}

impl ApplicationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client.application().list(Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                application_in,
                options,
            } => {
                let resp = client
                    .application()
                    .create(application_in.into_inner(), Some(options.into()))
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
                    .patch(id, application_patch.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
