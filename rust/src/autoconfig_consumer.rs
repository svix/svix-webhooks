use crate::{
    api::{EndpointOut, Svix, SvixOptions},
    api_internal,
    autoconfig::{decode_autoconfig_token_v1, AutoConfigError},
    error::Result,
    models::{AutoConfigSinkType, PollerV2CommitIn, PollerV2PollOut, SinkInCommon, SubscribeIn},
};

pub struct AutoConfigConsumer {
    app_id: String,
    sink_id: String,
    sink_in: SinkInCommon,
    svix: Svix,
}

impl AutoConfigConsumer {
    pub fn new(token: String, sink_in: SinkInCommon) -> std::result::Result<Self, AutoConfigError> {
        let content = decode_autoconfig_token_v1(&token)?;

        let svix = Svix::new(
            content.token_plaintext,
            Some(SvixOptions {
                server_url: Some(content.server_url),
                ..Default::default()
            }),
        );

        Ok(Self {
            app_id: content.app_id,
            sink_id: content.endpoint_id,
            sink_in,
            svix,
        })
    }

    pub async fn subscribe(&self) -> Result<EndpointOut> {
        let mut subscribe_in = SubscribeIn::new();
        subscribe_in.sink = Some(AutoConfigSinkType::Poller(self.sink_in.clone()));

        api_internal::endpoint_auto_config(self.svix.cfg())
            .update(self.app_id.clone(), self.sink_id.clone(), subscribe_in)
            .await
    }

    pub async fn receive(
        &self,
        consumer_id: String,
        options: Option<api_internal::message_pollerv2::MessagePollerv2ConsumerPollOptions>,
    ) -> Result<PollerV2PollOut> {
        api_internal::message_pollerv2(self.svix.cfg())
            .consumer_poll(
                self.app_id.clone(),
                self.sink_id.clone(),
                consumer_id,
                options,
            )
            .await
    }

    pub async fn commit(
        &self,
        consumer_id: String,
        offset: i32,
        options: Option<api_internal::message_pollerv2::MessagePollerv2ConsumerCommitOptions>,
    ) -> Result<()> {
        api_internal::message_pollerv2(self.svix.cfg())
            .consumer_commit(
                self.app_id.clone(),
                self.sink_id.clone(),
                consumer_id,
                PollerV2CommitIn::new(offset),
                options,
            )
            .await
    }
}
