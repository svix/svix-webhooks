// this file is @generated
use clap::{
    Args,
    Subcommand,
};
use svix::api::*;

#[derive(Args, Clone)]
pub struct ConnectorListOptions {
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

impl From<ConnectorListOptions> for svix::api::ConnectorListOptions {
    fn from(value: ConnectorListOptions) -> Self {
        let ConnectorListOptions {
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
pub struct ConnectorCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<ConnectorCreateOptions> for svix::api::ConnectorCreateOptions {
    fn from(value: ConnectorCreateOptions) -> Self {
        let ConnectorCreateOptions {
            idempotency_key,
        } = value;
        Self {
            idempotency_key,
        }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct ConnectorArgs {
    #[command(subcommand)]
    pub command: ConnectorCommands,
}

#[derive(Subcommand)]
pub enum ConnectorCommands {
    /// List all connectors for an application.
    List {
        #[clap(flatten)]
        options: ConnectorListOptions,
    },
    /// Create a new connector.
    Create {
        connector_in: crate::json::JsonOf<ConnectorIn>,
        #[clap(flatten)]
        options: ConnectorCreateOptions,
    },
    /// Get a connector.
    Get { id: String },
    /// Update a connector.
    Update {
        id: String,
        connector_update: crate::json::JsonOf<ConnectorUpdate>,
    },
    /// Delete a connector.
    Delete { id: String },
    /// Partially update a connector.
    Patch {
        id: String,
        connector_patch: Option<crate::json::JsonOf<ConnectorPatch>>,
    },
}

impl ConnectorCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List {
                options,
            } => {
                let resp = client.connector().list(Some(options.into())).await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::Create {
                connector_in,
                options,
            } => {
                let resp = client
                    .connector()
                    .create(
                        connector_in.into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::Get {
                id,
            } => {
                let resp = client.connector().get(id).await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::Update {
                id,
                connector_update,
            } => {
                let resp = client
                    .connector()
                    .update(
                        id,
                        connector_update.into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::Delete {
                id,
            } => {
                client.connector().delete(id).await?;
            }
            Self::Patch {
                id,
                connector_patch,
            } => {
                let resp = client
                    .connector()
                    .patch(
                        id,
                        connector_patch.unwrap_or_default().into_inner(),
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
