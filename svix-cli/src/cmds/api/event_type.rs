use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct EventTypeListOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// The sorting order of the returned items
    #[arg(long)]
    pub order: Option<Ordering>,
    /// When `true` archived (deleted but not expunged) items are included in the response.
    #[arg(long)]
    pub include_archived: Option<bool>,
    /// When `true` the full item (including the schema) is included in the response.
    #[arg(long)]
    pub with_content: Option<bool>,
}

impl From<EventTypeListOptions> for svix::api::EventTypeListOptions {
    fn from(value: EventTypeListOptions) -> Self {
        let EventTypeListOptions {
            limit,
            iterator,
            order,
            include_archived,
            with_content,
        } = value;
        Self {
            limit,
            iterator,
            order,
            include_archived,
            with_content,
        }
    }
}

#[derive(Args, Clone)]
pub struct EventTypeCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EventTypeCreateOptions> for svix::api::EventTypeCreateOptions {
    fn from(value: EventTypeCreateOptions) -> Self {
        let EventTypeCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EventTypeImportOpenapiOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EventTypeImportOpenapiOptions> for svix::api::EventTypeImportOpenapiOptions {
    fn from(value: EventTypeImportOpenapiOptions) -> Self {
        let EventTypeImportOpenapiOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EventTypeDeleteOptions {
    /// By default event types are archived when "deleted". Passing this to `true` deletes them entirely.
    #[arg(long)]
    pub expunge: Option<bool>,
}

impl From<EventTypeDeleteOptions> for svix::api::EventTypeDeleteOptions {
    fn from(value: EventTypeDeleteOptions) -> Self {
        let EventTypeDeleteOptions { expunge } = value;
        Self { expunge }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct EventTypeArgs {
    #[command(subcommand)]
    pub command: EventTypeCommands,
}

#[derive(Subcommand)]
pub enum EventTypeCommands {
    /// Return the list of event types.
    List {
        #[clap(flatten)]
        options: EventTypeListOptions,
    },
    /// Create new or unarchive existing event type.
    ///
    /// Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
    /// Endpoints filtering on the event type before archival will continue to filter on it.
    /// This operation does not preserve the description and schemas.
    Create {
        event_type_in: crate::json::JsonOf<EventTypeIn>,
        #[clap(flatten)]
        options: EventTypeCreateOptions,
    },
    /// Given an OpenAPI spec, create new or update existing event types.
    /// If an existing `archived` event type is updated, it will be unarchived.
    ///
    /// The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
    /// top-level.
    ImportOpenapi {
        event_type_import_open_api_in: Option<crate::json::JsonOf<EventTypeImportOpenApiIn>>,
        #[clap(flatten)]
        options: EventTypeImportOpenapiOptions,
    },
    /// Get an event type.
    Get { event_type_name: String },
    /// Update an event type.
    Update {
        event_type_name: String,
        event_type_update: crate::json::JsonOf<EventTypeUpdate>,
    },
    /// Archive an event type.
    ///
    /// Endpoints already configured to filter on an event type will continue to do so after archival.
    /// However, new messages can not be sent with it and endpoints can not filter on it.
    /// An event type can be unarchived with the
    /// [create operation](#operation/create_event_type_api_v1_event_type__post).
    Delete {
        event_type_name: String,
        #[clap(flatten)]
        options: EventTypeDeleteOptions,
    },
    /// Partially update an event type.
    Patch {
        event_type_name: String,
        event_type_patch: Option<crate::json::JsonOf<EventTypePatch>>,
    },
}

impl EventTypeCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client.event_type().list(Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                event_type_in,
                options,
            } => {
                let resp = client
                    .event_type()
                    .create(event_type_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ImportOpenapi {
                event_type_import_open_api_in,
                options,
            } => {
                let resp = client
                    .event_type()
                    .import_openapi(
                        event_type_import_open_api_in
                            .unwrap_or_default()
                            .into_inner(),
                        Some(options.into()),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { event_type_name } => {
                let resp = client.event_type().get(event_type_name).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                event_type_name,
                event_type_update,
            } => {
                let resp = client
                    .event_type()
                    .update(event_type_name, event_type_update.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete {
                event_type_name,
                options,
            } => {
                client
                    .event_type()
                    .delete(event_type_name, Some(options.into()))
                    .await?;
            }
            Self::Patch {
                event_type_name,
                event_type_patch,
            } => {
                let resp = client
                    .event_type()
                    .patch(
                        event_type_name,
                        event_type_patch.unwrap_or_default().into_inner(),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
