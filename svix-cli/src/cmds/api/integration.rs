// this file is @generated
use clap::{
    Args,
    Subcommand,
};
use svix::api::*;

#[derive(Args, Clone)]
pub struct IntegrationListOptions {
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

impl From<IntegrationListOptions> for svix::api::IntegrationListOptions {
    fn from(value: IntegrationListOptions) -> Self {
        let IntegrationListOptions {
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
pub struct IntegrationCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IntegrationCreateOptions> for svix::api::IntegrationCreateOptions {
    fn from(value: IntegrationCreateOptions) -> Self {
        let IntegrationCreateOptions {
            idempotency_key,
        } = value;
        Self {
            idempotency_key,
        }
    }
}

#[derive(Args, Clone)]
pub struct IntegrationRotateKeyOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<IntegrationRotateKeyOptions> for svix::api::IntegrationRotateKeyOptions {
    fn from(value: IntegrationRotateKeyOptions) -> Self {
        let IntegrationRotateKeyOptions {
            idempotency_key,
        } = value;
        Self {
            idempotency_key,
        }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
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
        integration_in: crate::json::JsonOf<IntegrationIn>,
        #[clap(flatten)]
        options: IntegrationCreateOptions,
    },
    /// Get an integration.
    Get { app_id: String, id: String },
    /// Update an integration.
    Update {
        app_id: String,
        id: String,
        integration_update: crate::json::JsonOf<IntegrationUpdate>,
    },
    /// Delete an integration.
    Delete { app_id: String, id: String },
    /// Get an integration's key.
    GetKey { app_id: String, id: String },
    /// Rotate the integration's key. The previous key will be immediately revoked.
    RotateKey {
        app_id: String,
        id: String,
        #[clap(flatten)]
        options: IntegrationRotateKeyOptions,
    },
}

impl IntegrationCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List {
                app_id,
                options,
            } => {
                let resp = client
                    .integration()
                    .list(
                        app_id,
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::Create {
                app_id,
                integration_in,
                options,
            } => {
                let resp = client
                    .integration()
                    .create(
                        app_id,
                        integration_in.into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::Get {
                app_id,
                id,
            } => {
                let resp = client
                    .integration()
                    .get(
                        app_id, id,
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::Update {
                app_id,
                id,
                integration_update,
            } => {
                let resp = client
                    .integration()
                    .update(
                        app_id,
                        id,
                        integration_update.into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::Delete {
                app_id,
                id,
            } => {
                client
                    .integration()
                    .delete(
                        app_id, id,
                    )
                    .await?;
            }
            Self::GetKey {
                app_id,
                id,
            } => {
                #[allow(deprecated)]
                let resp = client
                    .integration()
                    .get_key(
                        app_id, id,
                    )
                    .await?;
                crate::json::print_json_output(
                    &resp, color_mode,
                )?;
            }
            Self::RotateKey {
                app_id,
                id,
                options,
            } => {
                let resp = client
                    .integration()
                    .rotate_key(
                        app_id,
                        id,
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
