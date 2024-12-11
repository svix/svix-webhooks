use super::ListOptions;
use crate::{apis::message_attempt_api, error::Result, models::*, Configuration};

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
    pub tag: Option<String>,
    pub status: Option<MessageStatus>,
    pub status_code_class: Option<StatusCodeClass>,
    pub with_content: Option<bool>,
    pub endpoint_id: Option<String>,
}

#[derive(Default)]
pub struct MessageAttemptListByEndpointOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
    pub event_types: Option<Vec<String>>,
    // FIXME: make before and after actual dates
    /// RFC3339 date string
    pub before: Option<String>,
    /// RFC3339 date string
    pub after: Option<String>,
    pub channel: Option<String>,
    pub tag: Option<String>,
    pub status: Option<MessageStatus>,
    pub status_code_class: Option<StatusCodeClass>,
    pub with_content: Option<bool>,
    pub with_msg: Option<bool>,
    pub endpoint_id: Option<String>,
}

pub struct MessageAttempt<'a> {
    cfg: &'a Configuration,
}

impl<'a> MessageAttempt<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
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
            tag,
            status_code_class,
            endpoint_id,
            with_content,
        } = options.unwrap_or_default();
        message_attempt_api::v1_period_message_attempt_period_list_by_msg(
            self.cfg,
            message_attempt_api::V1PeriodMessageAttemptPeriodListByMsgParams {
                app_id,
                msg_id,
                iterator,
                limit,
                event_types,
                before,
                after,
                channel,
                tag,
                status,
                status_code_class,
                endpoint_id,
                with_content,
            },
        )
        .await
    }

    pub async fn list_by_endpoint(
        &self,
        app_id: String,
        endpoint_id: String,
        options: Option<MessageAttemptListByEndpointOptions>,
    ) -> Result<ListResponseMessageAttemptOut> {
        let MessageAttemptListByEndpointOptions {
            iterator,
            limit,
            event_types,
            before,
            after,
            channel,
            tag,
            status,
            status_code_class,
            endpoint_id: _,
            with_content,
            with_msg,
        } = options.unwrap_or_default();
        message_attempt_api::v1_period_message_attempt_period_list_by_endpoint(
            self.cfg,
            message_attempt_api::V1PeriodMessageAttemptPeriodListByEndpointParams {
                app_id,
                endpoint_id,
                iterator,
                limit,
                event_types,
                before,
                after,
                channel,
                tag,
                status,
                status_code_class,
                with_content,
                with_msg,
            },
        )
        .await
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
            event_types,
            before,
            after,
            channel,
            tag,
            status,
            status_code_class: _,
            with_content,
            endpoint_id: _,
        } = options.unwrap_or_default();
        message_attempt_api::v1_period_message_attempt_period_list_attempted_messages(
            self.cfg,
            message_attempt_api::V1PeriodMessageAttemptPeriodListAttemptedMessagesParams {
                app_id,
                endpoint_id,
                iterator,
                limit,
                before,
                after,
                channel,
                tag,
                status,
                with_content,
                event_types,
            },
        )
        .await
    }

    pub async fn list_attempted_destinations(
        &self,
        app_id: String,
        msg_id: String,
        options: Option<ListOptions>,
    ) -> Result<ListResponseMessageEndpointOut> {
        let ListOptions { iterator, limit } = options.unwrap_or_default();
        message_attempt_api::v1_period_message_attempt_period_list_attempted_destinations(
            self.cfg,
            message_attempt_api::V1PeriodMessageAttemptPeriodListAttemptedDestinationsParams {
                app_id,
                msg_id,
                iterator,
                limit,
            },
        )
        .await
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
            tag,
            status,
            status_code_class: _,
            endpoint_id: _,
            with_content: _,
        } = options.unwrap_or_default();
        message_attempt_api::v1_period_message_attempt_period_list_by_endpoint_deprecated(
            self.cfg,
            message_attempt_api::V1PeriodMessageAttemptPeriodListByEndpointDeprecatedParams {
                app_id,
                endpoint_id,
                msg_id,
                iterator,
                limit,
                event_types,
                before,
                after,
                channel,
                tag,
                status,
            },
        )
        .await
    }

    pub async fn get(
        &self,
        app_id: String,
        msg_id: String,
        attempt_id: String,
    ) -> Result<MessageAttemptOut> {
        message_attempt_api::v1_period_message_attempt_period_get(
            self.cfg,
            message_attempt_api::V1PeriodMessageAttemptPeriodGetParams {
                app_id,
                msg_id,
                attempt_id,
            },
        )
        .await
    }

    pub async fn resend(&self, app_id: String, msg_id: String, endpoint_id: String) -> Result<()> {
        message_attempt_api::v1_period_message_attempt_period_resend(
            self.cfg,
            message_attempt_api::V1PeriodMessageAttemptPeriodResendParams {
                app_id,
                msg_id,
                endpoint_id,
                idempotency_key: None,
            },
        )
        .await
    }

    pub async fn expunge_content(
        &self,
        app_id: String,
        msg_id: String,
        attempt_id: String,
    ) -> Result<()> {
        message_attempt_api::v1_period_message_attempt_period_expunge_content(
            self.cfg,
            message_attempt_api::V1PeriodMessageAttemptPeriodExpungeContentParams {
                app_id,
                msg_id,
                attempt_id,
            },
        )
        .await
    }
}
