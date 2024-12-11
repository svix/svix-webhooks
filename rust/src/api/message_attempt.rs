use super::ListOptions;
use crate::{apis::message_attempt_api, error::Result, models::*, Configuration};

#[derive(Default)]
pub struct MessageAttemptListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// Filter response based on the status of the attempt: Success (0), Pending
    /// (1), Failed (2), or Sending (3)
    pub status: Option<MessageStatus>,

    /// Filter response based on the HTTP status code
    pub status_code_class: Option<StatusCodeClass>,

    /// Filter response based on the channel
    pub channel: Option<String>,

    /// Filter response based on the tag
    pub tag: Option<String>,

    /// Filter the attempts based on the attempted endpoint
    pub endpoint_id: Option<String>,

    /// Only include items created before a certain date
    ///
    /// RFC3339 date string.
    pub before: Option<String>,

    /// Only include items created after a certain date
    ///
    /// RFC3339 date string.
    pub after: Option<String>,

    /// When `true` attempt content is included in the response
    pub with_content: Option<bool>,

    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>,
}

#[derive(Default)]
pub struct MessageAttemptListByEndpointOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// Filter response based on the status of the attempt: Success (0), Pending
    /// (1), Failed (2), or Sending (3)
    pub status: Option<MessageStatus>,

    /// Filter response based on the HTTP status code
    pub status_code_class: Option<StatusCodeClass>,

    /// Filter response based on the channel
    pub channel: Option<String>,

    /// Filter response based on the tag
    pub tag: Option<String>,

    /// Only include items created before a certain date
    ///
    /// RFC3339 date string.
    pub before: Option<String>,

    /// Only include items created after a certain date
    ///
    /// RFC3339 date string.
    pub after: Option<String>,

    /// When `true` attempt content is included in the response
    pub with_content: Option<bool>,

    /// When `true`, the message information is included in the response
    pub with_msg: Option<bool>,

    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>,
}

pub struct MessageAttempt<'a> {
    cfg: &'a Configuration,
}

impl<'a> MessageAttempt<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List attempts by message id
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days'
    /// worth of data relative to now or, if an iterator is provided, 90
    /// days before/after the time indicated by the iterator ID. If you
    /// require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
    pub async fn list_by_msg(
        &self,
        app_id: String,
        msg_id: String,
        options: Option<MessageAttemptListOptions>,
    ) -> Result<ListResponseMessageAttemptOut> {
        let MessageAttemptListOptions {
            limit,
            iterator,
            status,
            status_code_class,
            channel,
            tag,
            endpoint_id,
            before,
            after,
            with_content,
            event_types,
        } = options.unwrap_or_default();

        message_attempt_api::v1_period_message_attempt_period_list_by_msg(
            self.cfg,
            message_attempt_api::V1PeriodMessageAttemptPeriodListByMsgParams {
                app_id,
                msg_id,
                limit,
                iterator,
                status,
                status_code_class,
                channel,
                tag,
                endpoint_id,
                before,
                after,
                with_content,
                event_types,
            },
        )
        .await
    }

    /// List attempts by endpoint id
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days'
    /// worth of data relative to now or, if an iterator is provided, 90
    /// days before/after the time indicated by the iterator ID. If you
    /// require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
    pub async fn list_by_endpoint(
        &self,
        app_id: String,
        endpoint_id: String,
        options: Option<MessageAttemptListByEndpointOptions>,
    ) -> Result<ListResponseMessageAttemptOut> {
        let MessageAttemptListByEndpointOptions {
            limit,
            iterator,
            status,
            status_code_class,
            channel,
            tag,
            before,
            after,
            with_content,
            with_msg,
            event_types,
        } = options.unwrap_or_default();

        message_attempt_api::v1_period_message_attempt_period_list_by_endpoint(
            self.cfg,
            message_attempt_api::V1PeriodMessageAttemptPeriodListByEndpointParams {
                app_id,
                endpoint_id,
                limit,
                iterator,
                status,
                status_code_class,
                channel,
                tag,
                before,
                after,
                with_content,
                with_msg,
                event_types,
            },
        )
        .await
    }

    /// List messages for a particular endpoint. Additionally includes metadata
    /// about the latest message attempt.
    ///
    /// The `before` parameter lets you filter all items created before a
    /// certain date and is ignored if an iterator is passed.
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days'
    /// worth of data relative to now or, if an iterator is provided, 90
    /// days before/after the time indicated by the iterator ID. If you
    /// require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
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
                limit,
                iterator,
                channel,
                tag,
                status,
                before,
                after,
                with_content,
                event_types,
            },
        )
        .await
    }

    /// List endpoints attempted by a given message. Additionally includes
    /// metadata about the latest message attempt. By default, endpoints are
    /// listed in ascending order by ID.
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

    /// `msg_id`: Use a message id or a message `eventId`
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

    /// Resend a message to the specified endpoint.
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

    /// Deletes the given attempt's response body. Useful when an endpoint
    /// accidentally returned sensitive content.
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
