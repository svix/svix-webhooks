use crate::apis::configuration::Configuration;
use crate::apis::{
    application_api, authentication_api, endpoint_api, event_type_api, integration_api,
    message_api, message_attempt_api,
};
use crate::error::Result;
pub use crate::models::*;

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct SvixOptions {
    pub debug: bool,
    pub server_url: Option<String>,
}

pub struct Svix {
    cfg: Configuration,
}

impl Svix {
    pub fn new(token: String, options: Option<SvixOptions>) -> Self {
        let base_path = options.and_then(|x| x.server_url).unwrap_or_else(|| {
            match token.split('.').last() {
                Some("us") => "https://api.us.svix.com",
                Some("eu") => "https://api.eu.svix.com",
                Some("in") => "https://api.in.svix.com",
                _ => "https://api.svix.com",
            }
            .to_string()
        });
        let cfg = Configuration {
            base_path,
            user_agent: Some(format!("svix-libs/{CRATE_VERSION}/rust")),
            bearer_access_token: Some(token),
            ..Configuration::default()
        };

        Self { cfg }
    }

    pub fn authentication(&self) -> Authentication<'_> {
        Authentication::new(&self.cfg)
    }

    pub fn application(&self) -> Application<'_> {
        Application::new(&self.cfg)
    }

    pub fn endpoint(&self) -> Endpoint<'_> {
        Endpoint::new(&self.cfg)
    }

    pub fn integration(&self) -> Integration<'_> {
        Integration::new(&self.cfg)
    }

    pub fn event_type(&self) -> EventType<'_> {
        EventType::new(&self.cfg)
    }

    pub fn message(&self) -> Message<'_> {
        Message::new(&self.cfg)
    }

    pub fn message_attempt(&self) -> MessageAttempt<'_> {
        MessageAttempt::new(&self.cfg)
    }
}

#[derive(Default)]
pub struct PostOptions {
    pub idempotency_key: Option<String>,
}

pub struct Authentication<'a> {
    cfg: &'a Configuration,
}

impl<'a> Authentication<'a> {
    fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn dashboard_access(
        &self,
        app_id: String,
        options: Option<PostOptions>,
    ) -> Result<DashboardAccessOut> {
        let options = options.unwrap_or_default();
        Ok(
            authentication_api::get_dashboard_access_api_v1_auth_dashboard_access_app_id_post(
                self.cfg,
                authentication_api::GetDashboardAccessApiV1AuthDashboardAccessAppIdPostParams {
                    app_id,
                    idempotency_key: options.idempotency_key,
                },
            )
            .await?,
        )
    }

    pub async fn app_portal_access(
        &self,
        app_id: String,
        app_portal_access_in: AppPortalAccessIn,
        options: Option<PostOptions>,
    ) -> Result<AppPortalAccessOut> {
        let options = options.unwrap_or_default();
        Ok(
            authentication_api::get_app_portal_access_api_v1_auth_app_portal_access_app_id_post(
                self.cfg,
                authentication_api::GetAppPortalAccessApiV1AuthAppPortalAccessAppIdPostParams {
                    app_id,
                    app_portal_access_in,
                    idempotency_key: options.idempotency_key,
                },
            )
            .await?,
        )
    }

    pub async fn logout(&self, options: Option<PostOptions>) -> Result<()> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        Ok(authentication_api::logout_api_v1_auth_logout_post(
            self.cfg,
            authentication_api::LogoutApiV1AuthLogoutPostParams { idempotency_key },
        )
        .await?)
    }
}

#[derive(Default)]
pub struct ListOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Default)]
pub struct ApplicationListOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
    pub order: Option<Ordering>,
}

pub struct Application<'a> {
    cfg: &'a Configuration,
}

impl<'a> Application<'a> {
    fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list(
        &self,
        options: Option<ApplicationListOptions>,
    ) -> Result<ListResponseApplicationOut> {
        let ApplicationListOptions {
            iterator,
            limit,
            order,
        } = options.unwrap_or_default();
        Ok(application_api::list_applications_api_v1_app_get(
            self.cfg,
            application_api::ListApplicationsApiV1AppGetParams {
                iterator,
                limit,
                order,
                idempotency_key: None,
            },
        )
        .await?)
    }

    pub async fn create(
        &self,
        application_in: ApplicationIn,
        options: Option<PostOptions>,
    ) -> Result<ApplicationOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        Ok(application_api::create_application_api_v1_app_post(
            self.cfg,
            application_api::CreateApplicationApiV1AppPostParams {
                application_in,
                idempotency_key,
                get_if_exists: None,
            },
        )
        .await?)
    }

    pub async fn get_or_create(
        &self,
        application_in: ApplicationIn,
        options: Option<PostOptions>,
    ) -> Result<ApplicationOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        Ok(application_api::create_application_api_v1_app_post(
            self.cfg,
            application_api::CreateApplicationApiV1AppPostParams {
                application_in,
                idempotency_key,
                get_if_exists: Some(true),
            },
        )
        .await?)
    }

    pub async fn get(&self, app_id: String) -> Result<ApplicationOut> {
        Ok(application_api::get_application_api_v1_app_app_id_get(
            self.cfg,
            application_api::GetApplicationApiV1AppAppIdGetParams {
                app_id,
                idempotency_key: None,
            },
        )
        .await?)
    }

    pub async fn update(
        &self,
        app_id: String,
        application_in: ApplicationIn,
        options: Option<PostOptions>,
    ) -> Result<ApplicationOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        Ok(application_api::update_application_api_v1_app_app_id_put(
            self.cfg,
            application_api::UpdateApplicationApiV1AppAppIdPutParams {
                app_id,
                application_in,
                idempotency_key,
            },
        )
        .await?)
    }

    pub async fn delete(&self, app_id: String) -> Result<()> {
        Ok(
            application_api::delete_application_api_v1_app_app_id_delete(
                self.cfg,
                application_api::DeleteApplicationApiV1AppAppIdDeleteParams {
                    app_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }
}

#[derive(Default)]
pub struct EndpointListOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
    pub order: Option<Ordering>,
}

pub struct Endpoint<'a> {
    cfg: &'a Configuration,
}

impl<'a> Endpoint<'a> {
    fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list(
        &self,
        app_id: String,
        options: Option<EndpointListOptions>,
    ) -> Result<ListResponseEndpointOut> {
        let EndpointListOptions {
            iterator,
            limit,
            order,
        } = options.unwrap_or_default();
        Ok(endpoint_api::list_endpoints_api_v1_app_app_id_endpoint_get(
            self.cfg,
            endpoint_api::ListEndpointsApiV1AppAppIdEndpointGetParams {
                app_id,
                order,
                iterator,
                limit,
                idempotency_key: None,
            },
        )
        .await?)
    }

    pub async fn create(
        &self,
        app_id: String,
        endpoint_in: EndpointIn,
        options: Option<PostOptions>,
    ) -> Result<EndpointOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        Ok(
            endpoint_api::create_endpoint_api_v1_app_app_id_endpoint_post(
                self.cfg,
                endpoint_api::CreateEndpointApiV1AppAppIdEndpointPostParams {
                    app_id,
                    endpoint_in,
                    idempotency_key,
                },
            )
            .await?,
        )
    }

    pub async fn get(&self, app_id: String, endpoint_id: String) -> Result<EndpointOut> {
        Ok(
            endpoint_api::get_endpoint_api_v1_app_app_id_endpoint_endpoint_id_get(
                self.cfg,
                endpoint_api::GetEndpointApiV1AppAppIdEndpointEndpointIdGetParams {
                    app_id,
                    endpoint_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn update(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_update: EndpointUpdate,
        options: Option<PostOptions>,
    ) -> Result<EndpointOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        Ok(
            endpoint_api::update_endpoint_api_v1_app_app_id_endpoint_endpoint_id_put(
                self.cfg,
                endpoint_api::UpdateEndpointApiV1AppAppIdEndpointEndpointIdPutParams {
                    app_id,
                    endpoint_id,
                    endpoint_update,
                    idempotency_key,
                },
            )
            .await?,
        )
    }

    pub async fn delete(&self, app_id: String, endpoint_id: String) -> Result<()> {
        Ok(
            endpoint_api::delete_endpoint_api_v1_app_app_id_endpoint_endpoint_id_delete(
                self.cfg,
                endpoint_api::DeleteEndpointApiV1AppAppIdEndpointEndpointIdDeleteParams {
                    app_id,
                    endpoint_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn get_secret(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<EndpointSecretOut> {
        Ok(
            endpoint_api::get_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_get(
                self.cfg,
                endpoint_api::GetEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGetParams {
                    app_id,
                    endpoint_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn rotate_secret(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
    ) -> Result<()> {
        Ok(
            endpoint_api::rotate_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_rotate_post(
                self.cfg,
                endpoint_api::RotateEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretRotatePostParams {
                    app_id,
                    endpoint_id,
                    endpoint_secret_rotate_in,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn recover(
        &self,
        app_id: String,
        endpoint_id: String,
        recover_in: RecoverIn,
    ) -> Result<()> {
        endpoint_api::recover_failed_webhooks_api_v1_app_app_id_endpoint_endpoint_id_recover_post(
            self.cfg,
            endpoint_api::RecoverFailedWebhooksApiV1AppAppIdEndpointEndpointIdRecoverPostParams {
                app_id,
                endpoint_id,
                recover_in,
                idempotency_key: None,
            },
        )
        .await?;
        Ok(())
    }

    pub async fn get_headers(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<EndpointHeadersOut> {
        Ok(
            endpoint_api::get_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_get(
                self.cfg,
                endpoint_api::GetEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersGetParams {
                    app_id,
                    endpoint_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn update_headers(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_headers_in: EndpointHeadersIn,
    ) -> Result<()> {
        Ok(
            endpoint_api::update_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_put(
                self.cfg,
                endpoint_api::UpdateEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPutParams {
                    app_id,
                    endpoint_id,
                    endpoint_headers_in,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn patch_headers(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_headers_patch_in: EndpointHeadersPatchIn,
    ) -> Result<()> {
        Ok(
            endpoint_api::patch_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_patch(
                self.cfg,
                endpoint_api::PatchEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPatchParams {
                    app_id,
                    endpoint_id,
                    endpoint_headers_patch_in,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn get_stats(&self, app_id: String, endpoint_id: String) -> Result<EndpointStats> {
        Ok(
            endpoint_api::get_endpoint_stats_api_v1_app_app_id_endpoint_endpoint_id_stats_get(
                self.cfg,
                endpoint_api::GetEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGetParams {
                    app_id,
                    endpoint_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn replay_missing(
        &self,
        app_id: String,
        endpoint_id: String,
        replay_in: ReplayIn,
        options: Option<PostOptions>,
    ) -> Result<()> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        endpoint_api::replay_missing_webhooks_api_v1_app_app_id_endpoint_endpoint_id_replay_missing_post(
            self.cfg,
            endpoint_api::ReplayMissingWebhooksApiV1AppAppIdEndpointEndpointIdReplayMissingPostParams{
                app_id,
                endpoint_id,
                replay_in,
                idempotency_key,
            },
        )
        .await?;
        Ok(())
    }

    pub async fn transformation_get(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<EndpointTransformationOut> {
        Ok(endpoint_api::get_endpoint_transformation_api_v1_app_app_id_endpoint_endpoint_id_transformation_get(
            self.cfg,
            endpoint_api::GetEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationGetParams {
                app_id,
                endpoint_id,
                idempotency_key: None,
            }
        )
        .await?)
    }

    pub async fn transformation_partial_update(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> Result<()> {
        endpoint_api::set_endpoint_transformation_api_v1_app_app_id_endpoint_endpoint_id_transformation_patch(self.cfg, endpoint_api::SetEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationPatchParams {
            app_id,
            endpoint_id,
            endpoint_transformation_in,
            idempotency_key: None,
        }).await?;
        Ok(())
    }
}

pub type IntegrationListOptions = ListOptions;

pub struct Integration<'a> {
    cfg: &'a Configuration,
}

impl<'a> Integration<'a> {
    fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list(
        &self,
        app_id: String,
        options: Option<IntegrationListOptions>,
    ) -> Result<ListResponseIntegrationOut> {
        let IntegrationListOptions { iterator, limit } = options.unwrap_or_default();
        Ok(
            integration_api::list_integrations_api_v1_app_app_id_integration_get(
                self.cfg,
                integration_api::ListIntegrationsApiV1AppAppIdIntegrationGetParams {
                    app_id,
                    iterator,
                    limit,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn create(
        &self,
        app_id: String,
        integration_in: IntegrationIn,
        options: Option<PostOptions>,
    ) -> Result<IntegrationOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        Ok(
            integration_api::create_integration_api_v1_app_app_id_integration_post(
                self.cfg,
                integration_api::CreateIntegrationApiV1AppAppIdIntegrationPostParams {
                    app_id,
                    integration_in,
                    idempotency_key,
                },
            )
            .await?,
        )
    }

    pub async fn get(&self, app_id: String, integ_id: String) -> Result<IntegrationOut> {
        Ok(
            integration_api::get_integration_api_v1_app_app_id_integration_integ_id_get(
                self.cfg,
                integration_api::GetIntegrationApiV1AppAppIdIntegrationIntegIdGetParams {
                    app_id,
                    integ_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn update(
        &self,
        app_id: String,
        integ_id: String,
        integration_update: IntegrationUpdate,
        options: Option<PostOptions>,
    ) -> Result<IntegrationOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        Ok(
            integration_api::update_integration_api_v1_app_app_id_integration_integ_id_put(
                self.cfg,
                integration_api::UpdateIntegrationApiV1AppAppIdIntegrationIntegIdPutParams {
                    app_id,
                    integ_id,
                    integration_update,
                    idempotency_key,
                },
            )
            .await?,
        )
    }

    pub async fn delete(&self, app_id: String, integ_id: String) -> Result<()> {
        Ok(
            integration_api::delete_integration_api_v1_app_app_id_integration_integ_id_delete(
                self.cfg,
                integration_api::DeleteIntegrationApiV1AppAppIdIntegrationIntegIdDeleteParams {
                    app_id,
                    integ_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn get_key(&self, app_id: String, integ_id: String) -> Result<IntegrationKeyOut> {
        Ok(
            integration_api::get_integration_key_api_v1_app_app_id_integration_integ_id_key_get(
                self.cfg,
                integration_api::GetIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyGetParams {
                    app_id,
                    integ_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn rotate_key(&self, app_id: String, integ_id: String) -> Result<IntegrationKeyOut> {
        Ok(
            integration_api::rotate_integration_key_api_v1_app_app_id_integration_integ_id_key_rotate_post(
                self.cfg,
                integration_api::RotateIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyRotatePostParams {
                    app_id,
                    integ_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }
}

#[derive(Default)]
pub struct EventTypeListOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
    pub with_content: Option<bool>,
    pub include_archived: Option<bool>,
}

pub struct EventType<'a> {
    cfg: &'a Configuration,
}

impl<'a> EventType<'a> {
    fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list(
        &self,
        options: Option<EventTypeListOptions>,
    ) -> Result<ListResponseEventTypeOut> {
        let EventTypeListOptions {
            iterator,
            limit,
            with_content,
            include_archived,
        } = options.unwrap_or_default();
        Ok(event_type_api::list_event_types_api_v1_event_type_get(
            self.cfg,
            event_type_api::ListEventTypesApiV1EventTypeGetParams {
                iterator,
                limit,
                with_content,
                include_archived,
                idempotency_key: None,
            },
        )
        .await?)
    }

    pub async fn create(
        &self,
        event_type_in: EventTypeIn,
        options: Option<PostOptions>,
    ) -> Result<EventTypeOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        Ok(event_type_api::create_event_type_api_v1_event_type_post(
            self.cfg,
            event_type_api::CreateEventTypeApiV1EventTypePostParams {
                event_type_in,
                idempotency_key,
            },
        )
        .await?)
    }

    pub async fn get(&self, event_type_name: String) -> Result<EventTypeOut> {
        Ok(
            event_type_api::get_event_type_api_v1_event_type_event_type_name_get(
                self.cfg,
                event_type_api::GetEventTypeApiV1EventTypeEventTypeNameGetParams {
                    event_type_name,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn update(
        &self,
        event_type_name: String,
        event_type_update: EventTypeUpdate,
        options: Option<PostOptions>,
    ) -> Result<EventTypeOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        Ok(
            event_type_api::update_event_type_api_v1_event_type_event_type_name_put(
                self.cfg,
                event_type_api::UpdateEventTypeApiV1EventTypeEventTypeNamePutParams {
                    event_type_name,
                    event_type_update,
                    idempotency_key,
                },
            )
            .await?,
        )
    }

    pub async fn delete(&self, event_type_name: String) -> Result<()> {
        Ok(
            event_type_api::delete_event_type_api_v1_event_type_event_type_name_delete(
                self.cfg,
                event_type_api::DeleteEventTypeApiV1EventTypeEventTypeNameDeleteParams {
                    event_type_name,
                    expunge: None,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }
}

#[derive(Default)]
pub struct MessageListOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
    pub event_types: Option<Vec<String>>,
    // FIXME: make before and after actual dates
    /// RFC3339 date string
    pub before: Option<String>,
    /// RFC3339 date string
    pub after: Option<String>,
    pub channel: Option<String>,
}

pub struct Message<'a> {
    cfg: &'a Configuration,
}

impl<'a> Message<'a> {
    fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list(
        &self,
        app_id: String,
        options: Option<MessageListOptions>,
    ) -> Result<ListResponseMessageOut> {
        let MessageListOptions {
            iterator,
            limit,
            event_types,
            before,
            after,
            channel,
        } = options.unwrap_or_default();
        Ok(message_api::list_messages_api_v1_app_app_id_msg_get(
            self.cfg,
            message_api::ListMessagesApiV1AppAppIdMsgGetParams {
                app_id,
                iterator,
                limit,
                event_types,
                before,
                after,
                channel,
                idempotency_key: None,
            },
        )
        .await?)
    }

    pub async fn create(
        &self,
        app_id: String,
        message_in: MessageIn,
        options: Option<PostOptions>,
    ) -> Result<MessageOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        Ok(message_api::create_message_api_v1_app_app_id_msg_post(
            self.cfg,
            message_api::CreateMessageApiV1AppAppIdMsgPostParams {
                app_id,
                message_in,
                idempotency_key,
                with_content: None,
            },
        )
        .await?)
    }

    pub async fn get(&self, app_id: String, msg_id: String) -> Result<MessageOut> {
        Ok(message_api::get_message_api_v1_app_app_id_msg_msg_id_get(
            self.cfg,
            message_api::GetMessageApiV1AppAppIdMsgMsgIdGetParams {
                app_id,
                msg_id,
                idempotency_key: None,
            },
        )
        .await?)
    }

    pub async fn expunge_content(&self, app_id: String, msg_id: String) -> Result<()> {
        Ok(
            message_api::expunge_message_payload_api_v1_app_app_id_msg_msg_id_content_delete(
                self.cfg,
                message_api::ExpungeMessagePayloadApiV1AppAppIdMsgMsgIdContentDeleteParams {
                    msg_id,
                    app_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }
}

#[derive(Default)]
pub struct MessageAttemptListOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
    pub event_types: Option<Vec<String>>,
    // FIXME: make before and after actual dates
    /// RFC3339 date string
    pub before: Option<String>,
    /// RFC3339 date string
    pub after: Option<String>,
    pub channel: Option<String>,
    pub status: Option<MessageStatus>,
    pub status_code_class: Option<StatusCodeClass>,
}

pub struct MessageAttempt<'a> {
    cfg: &'a Configuration,
}

impl<'a> MessageAttempt<'a> {
    fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list_by_msg(
        &self,
        app_id: String,
        msg_id: String,
        options: Option<MessageAttemptListOptions>,
    ) -> Result<ListResponseMessageAttemptOut> {
        let MessageAttemptListOptions {
            iterator,
            limit,
            event_types,
            before,
            after,
            channel,
            status,
            status_code_class,
        } = options.unwrap_or_default();
        Ok(
            message_attempt_api::list_attempts_by_msg_api_v1_app_app_id_attempt_msg_msg_id_get(
                self.cfg,
                message_attempt_api::ListAttemptsByMsgApiV1AppAppIdAttemptMsgMsgIdGetParams {
                    app_id,
                    msg_id,
                    iterator,
                    limit,
                    event_types,
                    before,
                    after,
                    channel,
                    status,
                    status_code_class,
                    endpoint_id: None,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn list_by_endpoint(
        &self,
        app_id: String,
        endpoint_id: String,
        options: Option<MessageAttemptListOptions>,
    ) -> Result<ListResponseMessageAttemptOut> {
        let MessageAttemptListOptions {
            iterator,
            limit,
            event_types,
            before,
            after,
            channel,
            status,
            status_code_class,
        } = options.unwrap_or_default();
        Ok(
            message_attempt_api::list_attempts_by_endpoint_api_v1_app_app_id_attempt_endpoint_endpoint_id_get(
                self.cfg,
                message_attempt_api::ListAttemptsByEndpointApiV1AppAppIdAttemptEndpointEndpointIdGetParams {
                    app_id,
                    endpoint_id,
                    iterator,
                    limit,
                    event_types,
                    before,
                    after,
                    channel,
                    status,
                    status_code_class,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn list_attempted_messages(
        &self,
        app_id: String,
        endpoint_id: String,
        options: Option<MessageAttemptListOptions>,
    ) -> Result<ListResponseEndpointMessageOut> {
        let MessageAttemptListOptions {
            iterator,
            limit,
            event_types: _,
            before,
            after,
            channel,
            status,
            status_code_class: _,
        } = options.unwrap_or_default();
        Ok(
            message_attempt_api::list_attempted_messages_api_v1_app_app_id_endpoint_endpoint_id_msg_get(
                self.cfg,
                message_attempt_api::ListAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGetParams {
                    app_id,
                    endpoint_id,
                    iterator,
                    limit,
                    before,
                    after,
                    channel,
                    status,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn list_attempted_destinations(
        &self,
        app_id: String,
        msg_id: String,
        options: Option<ListOptions>,
    ) -> Result<ListResponseMessageEndpointOut> {
        let ListOptions { iterator, limit } = options.unwrap_or_default();
        Ok(
            message_attempt_api::list_attempted_destinations_api_v1_app_app_id_msg_msg_id_endpoint_get(
                self.cfg,
                message_attempt_api::ListAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGetParams {
                    app_id,
                    msg_id,
                    iterator,
                    limit,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn list_attempts_for_endpoint(
        &self,
        app_id: String,
        msg_id: String,
        endpoint_id: String,
        options: Option<MessageAttemptListOptions>,
    ) -> Result<ListResponseMessageAttemptEndpointOut> {
        let MessageAttemptListOptions {
            iterator,
            limit,
            event_types,
            before,
            after,
            channel,
            status,
            status_code_class: _,
        } = options.unwrap_or_default();
        Ok(message_attempt_api::list_attempts_for_endpoint_api_v1_app_app_id_msg_msg_id_endpoint_endpoint_id_attempt_get(
            self.cfg,
            message_attempt_api::ListAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGetParams{
                app_id,
                endpoint_id,
                msg_id,
                iterator,
                limit,
                event_types,
                before,
                after,
                channel,
                status,
                idempotency_key: None,
            },
        )
        .await?)
    }

    pub async fn get(
        &self,
        app_id: String,
        msg_id: String,
        attempt_id: String,
    ) -> Result<MessageAttemptOut> {
        Ok(
            message_attempt_api::get_attempt_api_v1_app_app_id_msg_msg_id_attempt_attempt_id_get(
                self.cfg,
                message_attempt_api::GetAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGetParams {
                    app_id,
                    msg_id,
                    attempt_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn resend(&self, app_id: String, msg_id: String, endpoint_id: String) -> Result<()> {
        Ok(
            message_attempt_api::resend_webhook_api_v1_app_app_id_msg_msg_id_endpoint_endpoint_id_resend_post(
                self.cfg,
                message_attempt_api::ResendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPostParams {
                    app_id,
                    msg_id,
                    endpoint_id,
                    idempotency_key: None,
                },
            )
            .await?,
        )
    }

    pub async fn expunge_content(
        &self,
        app_id: String,
        msg_id: String,
        attempt_id: String,
    ) -> Result<()> {
        Ok(
            message_attempt_api::expunge_attempt_content_api_v1_app_app_id_msg_msg_id_attempt_attempt_id_content_delete(
                self.cfg,
                message_attempt_api::ExpungeAttemptContentApiV1AppAppIdMsgMsgIdAttemptAttemptIdContentDeleteParams {
                    app_id,
                    msg_id,
                    attempt_id,
                    idempotency_key: None,
                },
            )
            .await?
        )
    }
}
