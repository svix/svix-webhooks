use super::PostOptions;
use crate::{apis::message_api, error::Result, models::*, Configuration};

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
    pub with_content: Option<bool>,
    pub tag: Option<String>,
}

pub struct Message<'a> {
    cfg: &'a Configuration,
}

impl<'a> Message<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list(
        &self,
        app_id: String,
        options: Option<MessageListOptions>,
    ) -> Result<ListResponseMessageOut> {
        let MessageListOptions {
            limit,
            iterator,
            channel,
            before,
            after,
            with_content,
            tag,
            event_types,
        } = options.unwrap_or_default();

        message_api::v1_period_message_period_list(
            self.cfg,
            message_api::V1PeriodMessagePeriodListParams {
                app_id,
                limit,
                iterator,
                channel,
                before,
                after,
                with_content,
                tag,
                event_types,
            },
        )
        .await
    }

    pub async fn create(
        &self,
        app_id: String,
        message_in: MessageIn,
        options: Option<PostOptions>,
    ) -> Result<MessageOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        message_api::v1_period_message_period_create(
            self.cfg,
            message_api::V1PeriodMessagePeriodCreateParams {
                app_id,
                message_in,
                idempotency_key,
                with_content: None,
            },
        )
        .await
    }

    pub async fn get(&self, app_id: String, msg_id: String) -> Result<MessageOut> {
        message_api::v1_period_message_period_get(
            self.cfg,
            message_api::V1PeriodMessagePeriodGetParams {
                app_id,
                msg_id,
                with_content: None,
            },
        )
        .await
    }

    pub async fn expunge_content(&self, app_id: String, msg_id: String) -> Result<()> {
        message_api::v1_period_message_period_expunge_content(
            self.cfg,
            message_api::V1PeriodMessagePeriodExpungeContentParams { msg_id, app_id },
        )
        .await
    }

    #[cfg(feature = "svix_beta")]
    pub async fn events(
        &self,
        params: message_api::V1PeriodMessagePeriodEventsParams,
    ) -> Result<crate::models::MessageEventsOut> {
        message_api::v1_period_message_period_events(self.cfg, params).await
    }

    #[cfg(feature = "svix_beta")]
    pub async fn events_subscription(
        &self,
        params: message_api::V1PeriodMessagePeriodEventsSubscriptionParams,
    ) -> Result<crate::models::MessageEventsOut> {
        message_api::v1_period_message_period_events_subscription(self.cfg, params).await
    }
}
