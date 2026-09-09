use crate::{
    api::{Svix, SvixOptions},
    api_internal,
    autoconfig::{decode_autoconfig_token, AutoConfigError, AutoConfigToken},
    error::{Error, Result},
    models::{
        AutoConfigSinkType, DestinationIn, DestinationInConfig, DestinationOut,
        DestinationOutConfig, DestinationStatus, EndpointOut, PollerV2CommitIn, PollerV2PollOut,
        SinkInCommon, SubscribeIn,
    },
};

// Re-exported so callers can name and construct the `options` arguments of
// `receive`/`commit`. The types are `pub` but live in the crate-private
// `api_internal` module, so without this they would be reachable yet
// unnameable.
pub use crate::api_internal::message_pollerv2::{
    MessagePollerv2ConsumerCommitOptions, MessagePollerv2ConsumerPollOptions,
};

pub struct AutoConfigConsumer {
    app_id: String,
    sink_id: Option<String>,
    autoconfig_id: Option<String>,
    sink_in: SinkInCommon,
    svix: Svix,
}

impl AutoConfigConsumer {
    pub fn new(token: String, sink_in: SinkInCommon) -> std::result::Result<Self, AutoConfigError> {
        let decoded = decode_autoconfig_token(&token)?;

        let (app_id, server_url, token_plaintext, sink_id, autoconfig_id) = match decoded {
            AutoConfigToken::V1(content) => (
                content.app_id,
                content.server_url,
                content.token_plaintext,
                Some(content.endpoint_id),
                None,
            ),
            AutoConfigToken::V2(content) => (
                content.app_id,
                content.server_url,
                content.token_plaintext,
                None,
                Some(content.autoconfig_id),
            ),
        };

        let svix = Svix::new(
            token_plaintext,
            Some(SvixOptions {
                server_url: Some(server_url),
                ..Default::default()
            }),
        );

        Ok(Self {
            app_id,
            sink_id,
            autoconfig_id,
            sink_in,
            svix,
        })
    }

    pub async fn subscribe(&mut self) -> Result<DestinationOut> {
        if let Some(autoconfig_id) = &self.autoconfig_id {
            let destination = api_internal::destination_autoconfig(self.svix.cfg())
                .subscribe(
                    self.app_id.clone(),
                    autoconfig_id.clone(),
                    sink_in_common_to_polling_destination(&self.sink_in),
                )
                .await?;
            self.sink_id = Some(destination.id.clone());
            return Ok(destination);
        }

        let mut subscribe_in = SubscribeIn::new();
        subscribe_in.sink = Some(AutoConfigSinkType::Poller(self.sink_in.clone()));

        let endpoint = api_internal::endpoint_auto_config_deprecated(self.svix.cfg())
            .update(
                self.app_id.clone(),
                self.sink_id.clone().expect("v1 tokens set sink_id"),
                subscribe_in,
            )
            .await?;

        Ok(destination_out_from_v1_endpoint(endpoint))
    }

    async fn get_sink_id(&mut self) -> Result<String> {
        if let Some(sink_id) = self.sink_id.clone() {
            // Already have the sink id from subscribe() or the v1 token
            return Ok(sink_id);
        }

        // Get the sink id from the autoconfig id (v2)
        api_internal::endpoint_autoconfig(self.svix.cfg())
            .get(
                self.app_id.clone(),
                self.autoconfig_id
                    .clone()
                    .expect("v2 tokens set autoconfig_id"),
            )
            .await?
            .dest_id
            .ok_or_else(|| {
                Error::Generic(
                    "autoconfig subscription is pending. Have you called subscribe()?".to_owned(),
                )
            })
    }

    pub async fn receive(
        &mut self,
        consumer_id: String,
        options: Option<api_internal::message_pollerv2::MessagePollerv2ConsumerPollOptions>,
    ) -> Result<PollerV2PollOut> {
        let sink_id = self.get_sink_id().await?;

        api_internal::message_pollerv2(self.svix.cfg())
            .consumer_poll(self.app_id.clone(), sink_id, consumer_id, options)
            .await
    }

    pub async fn commit(
        &mut self,
        consumer_id: String,
        offset: u64,
        options: Option<api_internal::message_pollerv2::MessagePollerv2ConsumerCommitOptions>,
    ) -> Result<()> {
        let sink_id = self.get_sink_id().await?;

        api_internal::message_pollerv2(self.svix.cfg())
            .consumer_commit(
                self.app_id.clone(),
                sink_id,
                consumer_id,
                PollerV2CommitIn::new(offset),
                options,
            )
            .await
    }
}

fn sink_in_common_to_polling_destination(sink: &SinkInCommon) -> DestinationIn {
    DestinationIn {
        uid: sink.uid.clone(),
        status: None,
        batch_size: None,
        max_wait_secs: None,
        event_types: sink
            .event_types
            .as_ref()
            .map(|types| types.iter().cloned().collect()),
        channels: sink
            .channels
            .as_ref()
            .map(|channels| channels.iter().cloned().collect()),
        metadata: sink.metadata.clone(),
        config: DestinationInConfig::PollingEndpoint,
    }
}

/// The v1 deprecated API returns [`EndpointOut`]. JS casts that to
/// [`DestinationOut`]; here we keep the fields that overlap so callers can
/// read `id` (used by receive/commit).
fn destination_out_from_v1_endpoint(endpoint: EndpointOut) -> DestinationOut {
    DestinationOut {
        id: endpoint.id,
        uid: endpoint.uid,
        status: if endpoint.disabled == Some(true) {
            DestinationStatus::Disabled
        } else {
            DestinationStatus::Enabled
        },
        current_iterator: String::new(),
        failure_reason: None,
        created_at: endpoint.created_at,
        updated_at: endpoint.updated_at,
        batch_size: 0,
        max_wait_secs: 0,
        event_types: endpoint
            .event_types
            .map(|types| types.into_iter().collect()),
        channels: endpoint
            .channels
            .map(|channels| channels.into_iter().collect()),
        next_retry_at: None,
        metadata: endpoint.metadata,
        config: DestinationOutConfig::PollingEndpoint,
    }
}
