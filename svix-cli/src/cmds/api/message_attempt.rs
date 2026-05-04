// this file is @generated
use clap::{Args, Subcommand};
use svix::api::*;

#[derive(Args, Clone)]
pub struct MessageAttemptListByEndpointOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), Sending (3), or Canceled (4)
    #[arg(long)]
    pub status: Option<MessageStatus>,
    /// Filter response based on the HTTP status code
    #[arg(long)]
    pub status_code_class: Option<StatusCodeClass>,
    /// Filter response based on the channel
    #[arg(long)]
    pub channel: Option<String>,
    /// Filter response based on the tag
    #[arg(long)]
    pub tag: Option<String>,
    /// Only include items created before a certain date
    #[arg(long)]
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    /// Only include items created after a certain date
    #[arg(long)]
    pub after: Option<chrono::DateTime<chrono::Utc>>,
    /// When `true` attempt content is included in the response
    #[arg(long)]
    pub with_content: Option<bool>,
    /// When `true`, the message information is included in the response
    ///
    /// Note that message payloads are never included in the response, regardless of this flag.
    #[arg(long)]
    pub with_msg: Option<bool>,
    /// When `true`, return the Canceled (4) status in attempts. If `false`, canceled attempts are returned as Success (0)
    #[arg(long)]
    pub expanded_statuses: Option<bool>,
    /// Filter response based on the event type
    #[arg(long)]
    pub event_types: Option<Vec<String>>,
}

impl From<MessageAttemptListByEndpointOptions> for svix::api::MessageAttemptListByEndpointOptions {
    fn from(value: MessageAttemptListByEndpointOptions) -> Self {
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
            expanded_statuses,
            event_types,
        } = value;
        Self {
            limit,
            iterator,
            status,
            status_code_class,
            channel,
            tag,
            before: before.map(|dt| dt.to_rfc3339()),
            after: after.map(|dt| dt.to_rfc3339()),
            with_content,
            with_msg,
            expanded_statuses,
            event_types,
        }
    }
}

#[derive(Args, Clone)]
pub struct MessageAttemptListByMsgOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), Sending (3), or Canceled (4)
    #[arg(long)]
    pub status: Option<MessageStatus>,
    /// Filter response based on the HTTP status code
    #[arg(long)]
    pub status_code_class: Option<StatusCodeClass>,
    /// Filter response based on the channel
    #[arg(long)]
    pub channel: Option<String>,
    /// Filter response based on the tag
    #[arg(long)]
    pub tag: Option<String>,
    /// Filter the attempts based on the attempted endpoint
    #[arg(long)]
    pub endpoint_id: Option<String>,
    /// Only include items created before a certain date
    #[arg(long)]
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    /// Only include items created after a certain date
    #[arg(long)]
    pub after: Option<chrono::DateTime<chrono::Utc>>,
    /// When `true` attempt content is included in the response
    #[arg(long)]
    pub with_content: Option<bool>,
    /// When `true`, return the Canceled (4) status in attempts. If `false`, canceled attempts are returned as Success (0)
    #[arg(long)]
    pub expanded_statuses: Option<bool>,
    /// Filter response based on the event type
    #[arg(long)]
    pub event_types: Option<Vec<String>>,
}

impl From<MessageAttemptListByMsgOptions> for svix::api::MessageAttemptListByMsgOptions {
    fn from(value: MessageAttemptListByMsgOptions) -> Self {
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
            expanded_statuses,
            event_types,
        } = value;
        Self {
            limit,
            iterator,
            status,
            status_code_class,
            channel,
            tag,
            endpoint_id,
            before: before.map(|dt| dt.to_rfc3339()),
            after: after.map(|dt| dt.to_rfc3339()),
            with_content,
            expanded_statuses,
            event_types,
        }
    }
}

#[derive(Args, Clone)]
pub struct MessageAttemptListAttemptedMessagesOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
    /// Filter response based on the channel
    #[arg(long)]
    pub channel: Option<String>,
    /// Filter response based on the message tags
    #[arg(long)]
    pub tag: Option<String>,
    /// Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), Sending (3), or Canceled (4)
    #[arg(long)]
    pub status: Option<MessageStatus>,
    /// Only include items created before a certain date
    #[arg(long)]
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    /// Only include items created after a certain date
    #[arg(long)]
    pub after: Option<chrono::DateTime<chrono::Utc>>,
    /// When `true` message payloads are included in the response
    #[arg(long)]
    pub with_content: Option<bool>,
    /// When `true`, return the Canceled (4) status in attempts. If `false`, canceled attempts are returned as Success (0)
    #[arg(long)]
    pub expanded_statuses: Option<bool>,
    /// Filter response based on the event type
    #[arg(long)]
    pub event_types: Option<Vec<String>>,
}

impl From<MessageAttemptListAttemptedMessagesOptions>
    for svix::api::MessageAttemptListAttemptedMessagesOptions
{
    fn from(value: MessageAttemptListAttemptedMessagesOptions) -> Self {
        let MessageAttemptListAttemptedMessagesOptions {
            limit,
            iterator,
            channel,
            tag,
            status,
            before,
            after,
            with_content,
            expanded_statuses,
            event_types,
        } = value;
        Self {
            limit,
            iterator,
            channel,
            tag,
            status,
            before: before.map(|dt| dt.to_rfc3339()),
            after: after.map(|dt| dt.to_rfc3339()),
            with_content,
            expanded_statuses,
            event_types,
        }
    }
}

#[derive(Args, Clone)]
pub struct MessageAttemptGetOptions {
    /// When `true`, return the Canceled (4) status in attempts. If `false`, canceled attempts are returned as Success (0)
    #[arg(long)]
    pub expanded_statuses: Option<bool>,
}

impl From<MessageAttemptGetOptions> for svix::api::MessageAttemptGetOptions {
    fn from(value: MessageAttemptGetOptions) -> Self {
        let MessageAttemptGetOptions { expanded_statuses } = value;
        Self { expanded_statuses }
    }
}

#[derive(Args, Clone)]
pub struct MessageAttemptListAttemptedDestinationsOptions {
    /// Limit the number of returned items
    #[arg(long)]
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    #[arg(long)]
    pub iterator: Option<String>,
}

impl From<MessageAttemptListAttemptedDestinationsOptions>
    for svix::api::MessageAttemptListAttemptedDestinationsOptions
{
    fn from(value: MessageAttemptListAttemptedDestinationsOptions) -> Self {
        let MessageAttemptListAttemptedDestinationsOptions { limit, iterator } = value;
        Self { limit, iterator }
    }
}

#[derive(Args, Clone)]
pub struct MessageAttemptResendOptions {
    #[arg(long)]
    pub idempotency_key: Option<String>,
}

impl From<MessageAttemptResendOptions> for svix::api::MessageAttemptResendOptions {
    fn from(value: MessageAttemptResendOptions) -> Self {
        let MessageAttemptResendOptions { idempotency_key } = value;
        Self { idempotency_key }
    }
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true, flatten_help = true)]
pub struct MessageAttemptArgs {
    #[command(subcommand)]
    pub command: MessageAttemptCommands,
}

#[derive(Subcommand)]
pub enum MessageAttemptCommands {
    /// List attempts by endpoint id
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
    /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
    /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message-attempt list-by-endpoint app_abc000000000000000000000000 ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"endpointId\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"id\": \"atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"msg\": {
      \"channels\": [\"project_123\",\"group_2\"],
      \"deliverAt\": \"2030-01-01T00:00:00Z\",
      \"eventId\": \"unique-identifier\",
      \"eventType\": \"user.signup\",
      \"id\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
      \"payload\": {
        \"email\": \"test@example.com\",
        \"type\": \"user.created\",
        \"username\": \"test_user\"
      },
      \"tags\": [\"...\"],
      \"timestamp\": \"2030-01-01T00:00:00Z\"
    },
    \"msgId\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"response\": \"{}\",
    \"responseDurationMs\": 123,
    \"responseStatusCode\": 200,
    \"status\": 0,
    \"statusText\": \"success\",
    \"timestamp\": \"2030-01-01T00:00:00Z\",
    \"triggerType\": 0,
    \"url\": \"https://example.com/webhook/\"
  }],
  \"done\": true,
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\"
}\n")]
    ListByEndpoint {
        app_id: String,
        endpoint_id: String,
        #[clap(flatten)]
        options: MessageAttemptListByEndpointOptions,
    },
    /// List attempts by message ID.
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
    /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
    /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message-attempt list-by-msg app_abc000000000000000000000000 msg_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"endpointId\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"id\": \"atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"msg\": {
      \"channels\": [\"project_123\",\"group_2\"],
      \"deliverAt\": \"2030-01-01T00:00:00Z\",
      \"eventId\": \"unique-identifier\",
      \"eventType\": \"user.signup\",
      \"id\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
      \"payload\": {
        \"email\": \"test@example.com\",
        \"type\": \"user.created\",
        \"username\": \"test_user\"
      },
      \"tags\": [\"...\"],
      \"timestamp\": \"2030-01-01T00:00:00Z\"
    },
    \"msgId\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"response\": \"{}\",
    \"responseDurationMs\": 123,
    \"responseStatusCode\": 200,
    \"status\": 0,
    \"statusText\": \"success\",
    \"timestamp\": \"2030-01-01T00:00:00Z\",
    \"triggerType\": 0,
    \"url\": \"https://example.com/webhook/\"
  }],
  \"done\": true,
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\"
}\n")]
    ListByMsg {
        app_id: String,
        msg_id: String,
        #[clap(flatten)]
        options: MessageAttemptListByMsgOptions,
    },
    /// List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.
    ///
    /// The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
    ///
    /// Note that by default this endpoint is limited to retrieving 90 days' worth of data
    /// relative to now or, if an iterator is provided, 90 days before/after the time indicated
    /// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
    /// set the `before` or `after` parameter as appropriate.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message-attempt list-attempted-messages app_abc000000000000000000000000 ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"channels\": [\"project_123\",\"group_2\"],
    \"deliverAt\": \"2030-01-01T00:00:00Z\",
    \"eventId\": \"unique-identifier\",
    \"eventType\": \"user.signup\",
    \"id\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"nextAttempt\": \"2030-01-01T00:00:00Z\",
    \"payload\": {
      \"email\": \"test@example.com\",
      \"type\": \"user.created\",
      \"username\": \"test_user\"
    },
    \"status\": 0,
    \"statusText\": \"success\",
    \"tags\": [\"...\"],
    \"timestamp\": \"2030-01-01T00:00:00Z\"
  }],
  \"done\": true,
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\"
}\n")]
    ListAttemptedMessages {
        app_id: String,
        endpoint_id: String,
        #[clap(flatten)]
        options: MessageAttemptListAttemptedMessagesOptions,
    },
    /// `msg_id`: Use a message id or a message `eventId`
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message-attempt get app_abc000000000000000000000000 msg_abc000000000000000000000000 atmpt_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"endpointId\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"id\": \"atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"msg\": {
    \"channels\": [\"project_123\",\"group_2\"],
    \"deliverAt\": \"2030-01-01T00:00:00Z\",
    \"eventId\": \"unique-identifier\",
    \"eventType\": \"user.signup\",
    \"id\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"payload\": {
      \"email\": \"test@example.com\",
      \"type\": \"user.created\",
      \"username\": \"test_user\"
    },
    \"tags\": [\"...\"],
    \"timestamp\": \"2030-01-01T00:00:00Z\"
  },
  \"msgId\": \"msg_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
  \"response\": \"{}\",
  \"responseDurationMs\": 123,
  \"responseStatusCode\": 200,
  \"status\": 0,
  \"statusText\": \"success\",
  \"timestamp\": \"2030-01-01T00:00:00Z\",
  \"triggerType\": 0,
  \"url\": \"https://example.com/webhook/\"
}\n")]
    Get {
        app_id: String,
        msg_id: String,
        attempt_id: String,
        #[clap(flatten)]
        options: MessageAttemptGetOptions,
    },
    /// Deletes the given attempt's response body.
    ///
    /// Useful when an endpoint accidentally returned sensitive content.
    /// The message can't be replayed or resent once its payload has been deleted or expired.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message-attempt expunge-content app_abc000000000000000000000000 msg_abc000000000000000000000000 atmpt_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    ExpungeContent {
        app_id: String,
        msg_id: String,
        attempt_id: String,
    },
    /// List endpoints attempted by a given message.
    ///
    /// Additionally includes metadata about the latest message attempt.
    /// By default, endpoints are listed in ascending order by ID.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message-attempt list-attempted-destinations app_abc000000000000000000000000 msg_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
  \"data\": [{
    \"channels\": [\"project_123\",\"group_2\"],
    \"createdAt\": \"2030-01-01T00:00:00Z\",
    \"description\": \"...\",
    \"disabled\": false,
    \"filterTypes\": [\"user.signup\",\"user.deleted\"],
    \"id\": \"ep_1srOrx2ZWZBpBUvZwXKQmoEYga2\",
    \"nextAttempt\": \"2030-01-01T00:00:00Z\",
    \"rateLimit\": 123,
    \"status\": 0,
    \"statusText\": \"success\",
    \"throttleRate\": 123,
    \"uid\": \"unique-identifier\",
    \"updatedAt\": \"2030-01-01T00:00:00Z\",
    \"url\": \"https://example.com/webhook/\",
    \"version\": 1
  }],
  \"done\": true,
  \"iterator\": \"iterator\",
  \"prevIterator\": \"-iterator\"
}\n")]
    ListAttemptedDestinations {
        app_id: String,
        msg_id: String,
        #[clap(flatten)]
        options: MessageAttemptListAttemptedDestinationsOptions,
    },
    /// Resend a message to the specified endpoint.
    #[command(help_template = concat!(
            "{about-with-newline}\n",
            "{usage-heading} {usage}\n\n",
            "Example: svix message-attempt resend app_abc000000000000000000000000 msg_abc000000000000000000000000 ep_abc000000000000000000000000\n",
            "{after-help}",
            "\n",
            "{all-args}",
        ))]
    #[command(after_help = "Example response:
{
}\n")]
    Resend {
        app_id: String,
        msg_id: String,
        endpoint_id: String,
        #[clap(flatten)]
        options: MessageAttemptResendOptions,
    },
}

impl MessageAttemptCommands {
    pub async fn exec(
        self,
        client: &Svix,
        color_mode: colored_json::ColorMode,
    ) -> anyhow::Result<()> {
        match self {
            Self::ListByEndpoint {
                app_id,
                endpoint_id,
                options,
            } => {
                let resp = client
                    .message_attempt()
                    .list_by_endpoint(app_id, endpoint_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ListByMsg {
                app_id,
                msg_id,
                options,
            } => {
                let resp = client
                    .message_attempt()
                    .list_by_msg(app_id, msg_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ListAttemptedMessages {
                app_id,
                endpoint_id,
                options,
            } => {
                let resp = client
                    .message_attempt()
                    .list_attempted_messages(app_id, endpoint_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Get {
                app_id,
                msg_id,
                attempt_id,
                options,
            } => {
                let resp = client
                    .message_attempt()
                    .get(app_id, msg_id, attempt_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::ExpungeContent {
                app_id,
                msg_id,
                attempt_id,
            } => {
                client
                    .message_attempt()
                    .expunge_content(app_id, msg_id, attempt_id)
                    .await?;
            }
            Self::ListAttemptedDestinations {
                app_id,
                msg_id,
                options,
            } => {
                let resp = client
                    .message_attempt()
                    .list_attempted_destinations(app_id, msg_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
            Self::Resend {
                app_id,
                msg_id,
                endpoint_id,
                options,
            } => {
                let resp = client
                    .message_attempt()
                    .resend(app_id, msg_id, endpoint_id, Some(options.into()))
                    .await?;
                crate::json::print_json_output(&resp, color_mode)?;
            }
        }

        Ok(())
    }
}
