use clap::{Args, Subcommand};
use colored_json::ColorMode;
use svix::api::{EventTypeImportOpenApiIn, EventTypeIn, EventTypePatch, EventTypeUpdate};

use crate::{
    cli_types::{event_type::EventTypeListOptions, PostOptions},
    json::JsonOf,
};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
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
        event_type_in: JsonOf<EventTypeIn>,
        #[clap(flatten)]
        post_options: Option<PostOptions>,
    },
    /// Given an OpenAPI spec, create new or update existing event types.
    ///
    /// If an existing `archived` event type is updated, it will be unarchived.
    ///
    /// The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
    /// top-level.
    ImportOpenapi {
        event_type_import_open_api_in: JsonOf<EventTypeImportOpenApiIn>,
        #[clap(flatten)]
        post_options: Option<PostOptions>,
    },
    /// Get an event type.
    Get { event_type_name: String },
    /// Update an event type.
    Update {
        event_type_name: String,
        event_type_update: JsonOf<EventTypeUpdate>,
    },
    /// Archive an event type.
    ///
    /// Endpoints already configured to filter on an event type will continue to do so after archival.
    /// However, new messages can not be sent with it and endpoints can not filter on it.
    /// An event type can be unarchived with the
    /// [create operation](#operation/create_event_type_api_v1_event_type__post).
    Delete {
        event_type_name: String,
        // FIXME: Not part of the Rust lib (yet)
        //#[clap(flatten)]
        //options: EventTypeDeleteOptions,
    },
    /// Partially update an event type.
    Patch {
        event_type_name: String,
        event_type_patch: JsonOf<EventTypePatch>,
    },
}

impl EventTypeCommands {
    pub async fn exec(self, client: &svix::api::Svix, color_mode: ColorMode) -> anyhow::Result<()> {
        match self {
            Self::List { options } => {
                let resp = client.event_type().list(Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                event_type_in,
                post_options,
            } => {
                let resp = client
                    .event_type()
                    .create(event_type_in.into_inner(), post_options.map(Into::into))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ImportOpenapi {
                event_type_import_open_api_in,
                post_options,
            } => {
                let resp = client
                    .event_type()
                    .import_openapi(
                        event_type_import_open_api_in.into_inner(),
                        post_options.map(Into::into),
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
            Self::Delete { event_type_name } => {
                client.event_type().delete(event_type_name).await?;
            }
            Self::Patch {
                event_type_name,
                event_type_patch,
            } => {
                let resp = client
                    .event_type()
                    .patch(event_type_name, event_type_patch.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
