// this file is @generated
use crate::{error::Result, models::*, Configuration};

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

#[derive(Default)]
pub struct MessageAttemptListByMsgOptions {
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
pub struct MessageAttemptListAttemptedMessagesOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// Filter response based on the channel
    pub channel: Option<String>,

    /// Filter response based on the message tags
    pub tag: Option<String>,

    /// Filter response based on the status of the attempt: Success (0), Pending
    /// (1), Failed (2), or Sending (3)
    pub status: Option<MessageStatus>,

    /// Only include items created before a certain date
    ///
    /// RFC3339 date string.
    pub before: Option<String>,

    /// Only include items created after a certain date
    ///
    /// RFC3339 date string.
    pub after: Option<String>,

    /// When `true` message payloads are included in the response
    pub with_content: Option<bool>,

    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>,
}

#[derive(Default)]
pub struct MessageAttemptListAttemptedDestinationsOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,
}

#[derive(Default)]
pub struct MessageAttemptResendOptions {
    pub idempotency_key: Option<String>,
}

pub struct MessageAttempt<'a> {
    cfg: &'a Configuration,
}

impl<'a> MessageAttempt<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
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

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/attempt/endpoint/{endpoint_id}",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("endpoint_id", endpoint_id)
        .with_optional_query_param("limit", limit)
        .with_optional_query_param("iterator", iterator)
        .with_optional_query_param("status", status)
        .with_optional_query_param("status_code_class", status_code_class)
        .with_optional_query_param("channel", channel)
        .with_optional_query_param("tag", tag)
        .with_optional_query_param("before", before)
        .with_optional_query_param("after", after)
        .with_optional_query_param("with_content", with_content)
        .with_optional_query_param("with_msg", with_msg)
        .with_optional_query_param("event_types", event_types)
        .execute(self.cfg)
        .await
    }

    /// List attempts by message ID.
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
        options: Option<MessageAttemptListByMsgOptions>,
    ) -> Result<ListResponseMessageAttemptOut> {
        let MessageAttemptListByMsgOptions {
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

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/attempt/msg/{msg_id}",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("msg_id", msg_id)
        .with_optional_query_param("limit", limit)
        .with_optional_query_param("iterator", iterator)
        .with_optional_query_param("status", status)
        .with_optional_query_param("status_code_class", status_code_class)
        .with_optional_query_param("channel", channel)
        .with_optional_query_param("tag", tag)
        .with_optional_query_param("endpoint_id", endpoint_id)
        .with_optional_query_param("before", before)
        .with_optional_query_param("after", after)
        .with_optional_query_param("with_content", with_content)
        .with_optional_query_param("event_types", event_types)
        .execute(self.cfg)
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
        options: Option<MessageAttemptListAttemptedMessagesOptions>,
    ) -> Result<ListResponseEndpointMessageOut> {
        let MessageAttemptListAttemptedMessagesOptions {
            limit,
            iterator,
            channel,
            tag,
            status,
            before,
            after,
            with_content,
            event_types,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/endpoint/{endpoint_id}/msg",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("endpoint_id", endpoint_id)
        .with_optional_query_param("limit", limit)
        .with_optional_query_param("iterator", iterator)
        .with_optional_query_param("channel", channel)
        .with_optional_query_param("tag", tag)
        .with_optional_query_param("status", status)
        .with_optional_query_param("before", before)
        .with_optional_query_param("after", after)
        .with_optional_query_param("with_content", with_content)
        .with_optional_query_param("event_types", event_types)
        .execute(self.cfg)
        .await
    }

    #[deprecated = "Use `list_by_msg` instead, setting the `endpoint_id` in `options`."]
    pub async fn list_attempts_for_endpoint(
        &self,
        app_id: String,
        msg_id: String,
        endpoint_id: String,
        options: Option<MessageAttemptListByMsgOptions>,
    ) -> Result<ListResponseMessageAttemptEndpointOut> {
        let MessageAttemptListByMsgOptions {
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

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/attempt",
        )
        .with_optional_query_param("limit", limit)
        .with_optional_query_param("iterator", iterator)
        .with_optional_query_param("channel", channel)
        .with_optional_query_param("tag", tag)
        .with_optional_query_param("status", status)
        .with_optional_query_param("before", before)
        .with_optional_query_param("after", after)
        .with_optional_query_param("event_types", event_types)
        .with_path_param("app_id", app_id.to_string())
        .with_path_param("msg_id", msg_id.to_string())
        .with_path_param("endpoint_id", endpoint_id.to_string())
        .execute(self.cfg)
        .await
    }

    /// `msg_id`: Use a message id or a message `eventId`
    pub async fn get(
        &self,
        app_id: String,
        msg_id: String,
        attempt_id: String,
    ) -> Result<MessageAttemptOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("msg_id", msg_id)
        .with_path_param("attempt_id", attempt_id)
        .execute(self.cfg)
        .await
    }

    /// Deletes the given attempt's response body.
    ///
    /// Useful when an endpoint accidentally returned sensitive content.
    /// The message can't be replayed or resent once its payload has been
    /// deleted or expired.
    pub async fn expunge_content(
        &self,
        app_id: String,
        msg_id: String,
        attempt_id: String,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/content",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("msg_id", msg_id)
        .with_path_param("attempt_id", attempt_id)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// List endpoints attempted by a given message.
    ///
    /// Additionally includes metadata about the latest message attempt.
    /// By default, endpoints are listed in ascending order by ID.
    pub async fn list_attempted_destinations(
        &self,
        app_id: String,
        msg_id: String,
        options: Option<MessageAttemptListAttemptedDestinationsOptions>,
    ) -> Result<ListResponseMessageEndpointOut> {
        let MessageAttemptListAttemptedDestinationsOptions { limit, iterator } =
            options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/msg/{msg_id}/endpoint",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("msg_id", msg_id)
        .with_optional_query_param("limit", limit)
        .with_optional_query_param("iterator", iterator)
        .execute(self.cfg)
        .await
    }

    /// Resend a message to the specified endpoint.
    pub async fn resend(
        &self,
        app_id: String,
        msg_id: String,
        endpoint_id: String,
        options: Option<MessageAttemptResendOptions>,
    ) -> Result<()> {
        let MessageAttemptResendOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/resend",
        )
        .with_path_param("app_id", app_id)
        .with_path_param("msg_id", msg_id)
        .with_path_param("endpoint_id", endpoint_id)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
