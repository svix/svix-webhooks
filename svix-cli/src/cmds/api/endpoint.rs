use clap::{Args, Subcommand};
use colored_json::ColorMode;
use svix::api::{
    EndpointHeadersIn, EndpointHeadersPatchIn, EndpointIn, EndpointPatch, EndpointSecretRotateIn,
    EndpointTransformationIn, EndpointUpdate, EventExampleIn, RecoverIn, ReplayIn,
};

use crate::{
    cli_types::{
        endpoint::{EndpointListOptions, EndpointStatsOptions},
        PostOptions,
    },
    json::JsonOf,
};

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct EndpointArgs {
    #[command(subcommand)]
    pub command: EndpointCommands,
}

#[derive(Subcommand)]
pub enum EndpointCommands {
    /// List the application's endpoints.
    List {
        app_id: String,
        #[clap(flatten)]
        options: EndpointListOptions,
    },
    /// Create a new endpoint for the application.
    ///
    /// When `secret` is `null` the secret is automatically generated (recommended)
    Create {
        app_id: String,
        endpoint_in: JsonOf<EndpointIn>,
        #[clap(flatten)]
        post_options: Option<PostOptions>,
    },
    /// Get an endpoint.
    Get { app_id: String, id: String },
    /// Update an endpoint.
    Update {
        app_id: String,
        id: String,
        endpoint_update: JsonOf<EndpointUpdate>,
    },
    /// Delete an endpoint.
    Delete { app_id: String, id: String },
    /// Partially update an endpoint.
    Patch {
        app_id: String,
        id: String,
        endpoint_patch: JsonOf<EndpointPatch>,
    },
    /// Get the additional headers to be sent with the webhook
    GetHeaders { app_id: String, id: String },
    /// Set the additional headers to be sent with the webhook
    UpdateHeaders {
        app_id: String,
        id: String,
        endpoint_headers_in: JsonOf<EndpointHeadersIn>,
    },
    /// Partially set the additional headers to be sent with the webhook
    PatchHeaders {
        app_id: String,
        id: String,
        endpoint_headers_patch_in: JsonOf<EndpointHeadersPatchIn>,
    },
    /// Resend all failed messages since a given time.
    ///
    /// Messages that were sent successfully, even if failed initially, are not resent.
    Recover {
        app_id: String,
        id: String,
        recover_in: JsonOf<RecoverIn>,
        // FIXME: Not in the Rust lib (yet)
        //#[clap(flatten)]
        //post_options: Option<PostOptions>,
    },
    /// Replays messages to the endpoint.
    ///
    /// Only messages that were created after `since` will be sent.
    /// Messages that were previously sent to the endpoint are not resent.
    Replay {
        app_id: String,
        id: String,
        replay_in: JsonOf<ReplayIn>,
        #[clap(flatten)]
        post_options: Option<PostOptions>,
    },
    /// Get the endpoint's signing secret.
    ///
    /// This is used to verify the authenticity of the webhook.
    /// For more information please refer to
    /// [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    GetSecret { app_id: String, id: String },
    /// Rotates the endpoint's signing secret.
    ///
    /// The previous secret will remain valid for the next 24 hours.
    RotateSecret {
        app_id: String,
        id: String,
        endpoint_secret_rotate_in: JsonOf<EndpointSecretRotateIn>,
        // FIXME: Not in the Rust lib (yet)
        //#[clap(flatten)]
        //post_options: Option<PostOptions>,
    },
    /// Send an example message for an event
    SendExample {
        app_id: String,
        id: String,
        event_example_in: JsonOf<EventExampleIn>,
        #[clap(flatten)]
        post_options: Option<PostOptions>,
    },
    /// Get basic statistics for the endpoint.
    GetStats {
        app_id: String,
        id: String,

        #[clap(flatten)]
        options: EndpointStatsOptions,
    },
    /// Get the transformation code associated with this endpoint
    TransformationGet { app_id: String, id: String },
    /// Set or unset the transformation code associated with this endpoint
    TransformationPartialUpdate {
        app_id: String,
        id: String,
        endpoint_transformation_in: JsonOf<EndpointTransformationIn>,
    },
}

impl EndpointCommands {
    pub async fn exec(self, client: &svix::api::Svix, color_mode: ColorMode) -> anyhow::Result<()> {
        match self {
            Self::List { app_id, options } => {
                let resp = client.endpoint().list(app_id, Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                app_id,
                endpoint_in,
                post_options,
            } => {
                let resp = client
                    .endpoint()
                    .create(
                        app_id,
                        endpoint_in.into_inner(),
                        post_options.map(Into::into),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get { app_id, id } => {
                let resp = client.endpoint().get(app_id, id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Update {
                app_id,
                id,
                endpoint_update,
            } => {
                let resp = client
                    .endpoint()
                    .update(app_id, id, endpoint_update.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Delete { app_id, id } => {
                client.endpoint().delete(app_id, id).await?;
            }
            Self::Patch {
                app_id,
                id,
                endpoint_patch,
            } => {
                let resp = client
                    .endpoint()
                    .patch(app_id, id, endpoint_patch.into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetHeaders { app_id, id } => {
                let resp = client.endpoint().get_headers(app_id, id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::UpdateHeaders {
                app_id,
                id,
                endpoint_headers_in,
            } => {
                client
                    .endpoint()
                    .update_headers(app_id, id, endpoint_headers_in.into_inner())
                    .await?;
            }
            Self::PatchHeaders {
                app_id,
                id,
                endpoint_headers_patch_in,
            } => {
                client
                    .endpoint()
                    .patch_headers(app_id, id, endpoint_headers_patch_in.into_inner())
                    .await?;
            }
            Self::Recover {
                app_id,
                id,
                recover_in,
            } => {
                client
                    .endpoint()
                    .recover(app_id, id, recover_in.into_inner())
                    .await?;
            }
            Self::Replay {
                app_id,
                id,
                replay_in,
                post_options,
            } => {
                client
                    .endpoint()
                    .replay_missing(
                        app_id,
                        id,
                        replay_in.into_inner(),
                        post_options.map(Into::into),
                    )
                    .await?;
            }
            Self::GetSecret { app_id, id } => {
                let resp = client.endpoint().get_secret(app_id, id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::RotateSecret {
                app_id,
                id,
                endpoint_secret_rotate_in,
            } => {
                client
                    .endpoint()
                    .rotate_secret(app_id, id, endpoint_secret_rotate_in.into_inner())
                    .await?;
            }
            Self::SendExample {
                app_id,
                id,
                event_example_in,
                post_options,
            } => {
                let resp = client
                    .endpoint()
                    .send_example(
                        app_id,
                        id,
                        event_example_in.into_inner(),
                        post_options.map(Into::into),
                    )
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetStats {
                app_id,
                id,
                options,
            } => {
                let resp = client
                    .endpoint()
                    .get_stats(app_id, id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::TransformationGet { app_id, id } => {
                let resp = client.endpoint().transformation_get(app_id, id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::TransformationPartialUpdate {
                app_id,
                id,
                endpoint_transformation_in,
            } => {
                client
                    .endpoint()
                    .transformation_partial_update(
                        app_id,
                        id,
                        endpoint_transformation_in.into_inner(),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
