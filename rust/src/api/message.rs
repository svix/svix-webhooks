use super::PostOptions;
use crate::{apis::message_api, error::Result, models::*, Configuration};

#[derive(Default)]
pub struct MessageListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// Filter response based on the channel
    pub channel: Option<String>,

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

    /// Filter messages matching the provided tag
    pub tag: Option<String>,

    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>,
}

pub struct Message<'a> {
    cfg: &'a Configuration,
}

impl<'a> Message<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List all of the application's messages.
    ///
    /// The `before` and `after` parameters let you filter all items created
    /// before or after a certain date. These can be used alongside an iterator
    /// to paginate over results within a certain window.
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days'
    /// worth of data relative to now or, if an iterator is provided, 90
    /// days before/after the time indicated by the iterator ID. If you
    /// require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
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

    /// Creates a new message and dispatches it to all of the application's
    /// endpoints.
    ///
    /// The `eventId` is an optional custom unique ID. It's verified to be
    /// unique only up to a day, after that no verification will be made. If
    /// a message with the same `eventId` already exists for the application, a
    /// 409 conflict error will be returned.
    ///
    /// The `eventType` indicates the type and schema of the event. All messages
    /// of a certain `eventType` are expected to have the same schema. Endpoints
    /// can choose to only listen to specific event types. Messages can also
    /// have `channels`, which similar to event types let endpoints filter by
    /// them. Unlike event types, messages can have multiple channels, and
    /// channels don't imply a specific message content or schema.
    ///
    /// The `payload` property is the webhook's body (the actual webhook
    /// message). Svix supports payload sizes of up to ~350kb, though it's
    /// generally a good idea to keep webhook payloads small, probably no larger
    /// than 40kb.
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

    /// Get a message by its ID or eventID.
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

    /// Delete the given message's payload. Useful in cases when a message was
    /// accidentally sent with sensitive content.
    ///
    /// The message can't be replayed or resent once its payload has been
    /// deleted or expired.
    pub async fn expunge_content(&self, app_id: String, msg_id: String) -> Result<()> {
        message_api::v1_period_message_period_expunge_content(
            self.cfg,
            message_api::V1PeriodMessagePeriodExpungeContentParams { app_id, msg_id },
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
