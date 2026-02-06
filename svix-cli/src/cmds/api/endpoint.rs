// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct EndpointListOptions {
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

impl From<EndpointListOptions> for svix::api::EndpointListOptions {
    fn from(value: EndpointListOptions) -> Self {
        let EndpointListOptions {
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
pub struct EndpointCreateOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EndpointCreateOptions> for svix::api::EndpointCreateOptions {
    fn from(value: EndpointCreateOptions) -> Self {
        let EndpointCreateOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EndpointBulkReplayOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EndpointBulkReplayOptions> for svix::api::EndpointBulkReplayOptions {
    fn from(value: EndpointBulkReplayOptions) -> Self {
        let EndpointBulkReplayOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EndpointRecoverOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EndpointRecoverOptions> for svix::api::EndpointRecoverOptions {
    fn from(value: EndpointRecoverOptions) -> Self {
        let EndpointRecoverOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EndpointReplayMissingOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EndpointReplayMissingOptions> for svix::api::EndpointReplayMissingOptions {
    fn from(value: EndpointReplayMissingOptions) -> Self {
        let EndpointReplayMissingOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EndpointRotateSecretOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EndpointRotateSecretOptions> for svix::api::EndpointRotateSecretOptions {
    fn from(value: EndpointRotateSecretOptions) -> Self {
        let EndpointRotateSecretOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EndpointSendExampleOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<EndpointSendExampleOptions> for svix::api::EndpointSendExampleOptions {
    fn from(value: EndpointSendExampleOptions) -> Self {
        let EndpointSendExampleOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args, Clone)]
pub struct EndpointGetStatsOptions {
    /// Filter the range to data starting from this date.
    #[arg(long)]
    pub since: Option<chrono::DateTime<chrono::Utc>>,
    /// Filter the range to data ending by this date.
    #[arg(long)]
    pub until: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<EndpointGetStatsOptions> for svix::api::EndpointGetStatsOptions {
    fn from(value: EndpointGetStatsOptions) -> Self {
        let EndpointGetStatsOptions { since, until } = value;
        Self {
            since: since.map(|dt| dt.to_rfc3339()),
            until: until.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
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
    /// When `secret` is `null` the secret is automatically generated (recommended).
    Create {
        app_id: String,
        endpoint_in: crate::json::JsonOf<EndpointIn>,
        #[clap(flatten)]
        options: EndpointCreateOptions,
    },
    /// Get an endpoint.
    Get { app_id: String, id: String },
    /// Update an endpoint.
    Update {
        app_id: String,
        id: String,
        endpoint_update: crate::json::JsonOf<EndpointUpdate>,
    },
    /// Delete an endpoint.
    Delete { app_id: String, id: String },
    /// Partially update an endpoint.
    Patch {
        app_id: String,
        id: String,
        endpoint_patch: Option<crate::json::JsonOf<EndpointPatch>>,
    },
    /// Bulk replay messages sent to the endpoint.
    ///
    /// Only messages that were created after `since` will be sent.
    /// This will replay both successful, and failed messages
    ///
    /// A completed task will return a payload like the following:
    /// ```json
    /// {
    ///   "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
    ///   "status": "finished",
    ///   "task": "endpoint.bulk-replay",
    ///   "data": {
    ///     "messagesSent": 2
    ///   }
    /// }
    /// ```
    BulkReplay {
        app_id: String,
        id: String,
        bulk_replay_in: crate::json::JsonOf<BulkReplayIn>,
        #[clap(flatten)]
        options: EndpointBulkReplayOptions,
    },
    /// Get the additional headers to be sent with the webhook.
    GetHeaders { app_id: String, id: String },
    /// Set the additional headers to be sent with the webhook.
    UpdateHeaders {
        app_id: String,
        id: String,
        endpoint_headers_in: crate::json::JsonOf<EndpointHeadersIn>,
    },
    /// Partially set the additional headers to be sent with the webhook.
    PatchHeaders {
        app_id: String,
        id: String,
        endpoint_headers_patch_in: crate::json::JsonOf<EndpointHeadersPatchIn>,
    },
    /// Resend all failed messages since a given time.
    ///
    /// Messages that were sent successfully, even if failed initially, are not resent.
    ///
    /// A completed task will return a payload like the following:
    /// ```json
    /// {
    ///   "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
    ///   "status": "finished",
    ///   "task": "endpoint.recover",
    ///   "data": {
    ///     "messagesSent": 2
    ///   }
    /// }
    /// ```
    Recover {
        app_id: String,
        id: String,
        recover_in: crate::json::JsonOf<RecoverIn>,
        #[clap(flatten)]
        options: EndpointRecoverOptions,
    },
    /// Replays messages to the endpoint.
    ///
    /// Only messages that were created after `since` will be sent.
    /// Messages that were previously sent to the endpoint are not resent.
    ///
    /// A completed task will return a payload like the following:
    /// ```json
    /// {
    ///   "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
    ///   "status": "finished",
    ///   "task": "endpoint.replay",
    ///   "data": {
    ///     "messagesSent": 2
    ///   }
    /// }
    /// ```
    ReplayMissing {
        app_id: String,
        id: String,
        replay_in: crate::json::JsonOf<ReplayIn>,
        #[clap(flatten)]
        options: EndpointReplayMissingOptions,
    },
    /// Get the endpoint's signing secret.
    ///
    /// This is used to verify the authenticity of the webhook.
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
    GetSecret { app_id: String, id: String },
    /// Rotates the endpoint's signing secret.
    ///
    /// The previous secret will remain valid for the next 24 hours.
    RotateSecret {
        app_id: String,
        id: String,
        endpoint_secret_rotate_in: Option<crate::json::JsonOf<EndpointSecretRotateIn>>,
        #[clap(flatten)]
        options: EndpointRotateSecretOptions,
    },
    /// Send an example message for an event.
    SendExample {
        app_id: String,
        id: String,
        event_example_in: crate::json::JsonOf<EventExampleIn>,
        #[clap(flatten)]
        options: EndpointSendExampleOptions,
    },
    /// Get basic statistics for the endpoint.
    GetStats {
        app_id: String,
        id: String,
        #[clap(flatten)]
        options: EndpointGetStatsOptions,
    },
    /// Get the transformation code associated with this endpoint.
    TransformationGet { app_id: String, id: String },
    /// Set or unset the transformation code associated with this endpoint.
    PatchTransformation {
        app_id: String,
        id: String,
        endpoint_transformation_patch: Option<crate::json::JsonOf<EndpointTransformationPatch>>,
    },
    /// This operation was renamed to `set-transformation`.
    TransformationPartialUpdate {
        app_id: String,
        id: String,
        endpoint_transformation_in: Option<crate::json::JsonOf<EndpointTransformationIn>>,
    },
}

impl EndpointCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::List { app_id, options } => {
                let resp = client.endpoint().list(app_id, Some(options.into())).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Create {
                app_id,
                endpoint_in,
                options,
            } => {
                let resp = client
                    .endpoint()
                    .create(app_id, endpoint_in.into_inner(), Some(options.into()))
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
                    .patch(app_id, id, endpoint_patch.unwrap_or_default().into_inner())
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::BulkReplay {
                app_id,
                id,
                bulk_replay_in,
                options,
            } => {
                let resp = client
                    .endpoint()
                    .bulk_replay(
                        app_id,
                        id,
                        bulk_replay_in.into_inner(),
                        Some(options.into()),
                    )
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
                options,
            } => {
                let resp = client
                    .endpoint()
                    .recover(app_id, id, recover_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ReplayMissing {
                app_id,
                id,
                replay_in,
                options,
            } => {
                let resp = client
                    .endpoint()
                    .replay_missing(app_id, id, replay_in.into_inner(), Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::GetSecret { app_id, id } => {
                let resp = client.endpoint().get_secret(app_id, id).await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::RotateSecret {
                app_id,
                id,
                endpoint_secret_rotate_in,
                options,
            } => {
                client
                    .endpoint()
                    .rotate_secret(
                        app_id,
                        id,
                        endpoint_secret_rotate_in.unwrap_or_default().into_inner(),
                        Some(options.into()),
                    )
                    .await?;
            }
            Self::SendExample {
                app_id,
                id,
                event_example_in,
                options,
            } => {
                let resp = client
                    .endpoint()
                    .send_example(
                        app_id,
                        id,
                        event_example_in.into_inner(),
                        Some(options.into()),
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
            Self::PatchTransformation {
                app_id,
                id,
                endpoint_transformation_patch,
            } => {
                client
                    .endpoint()
                    .patch_transformation(
                        app_id,
                        id,
                        endpoint_transformation_patch
                            .unwrap_or_default()
                            .into_inner(),
                    )
                    .await?;
            }
            Self::TransformationPartialUpdate {
                app_id,
                id,
                endpoint_transformation_in,
            } => {
                #[allow(deprecated)]
                client
                    .endpoint()
                    .transformation_partial_update(
                        app_id,
                        id,
                        endpoint_transformation_in.unwrap_or_default().into_inner(),
                    )
                    .await?;
            }
        }

        Ok(())
    }
}
