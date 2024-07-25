use rdkafka::{
    consumer::StreamConsumer, error::KafkaResult, producer::FutureProducer, ClientConfig,
};
use serde::Deserialize;
use svix_bridge_types::{ReceiverOutput, SenderInput, SenderOutputOpts, TransformationConfig};

use crate::{input::KafkaConsumer, KafkaProducer, Result};

#[derive(Clone, Deserialize)]
#[serde(tag = "type")]
pub enum KafkaInputOpts {
    // Single-variant enum so we can require the "type": "kafka" field in deserialization
    #[serde(rename = "kafka")]
    Inner {
        /// Comma-separated list of addresses.
        ///
        /// Example: `localhost:9094`
        #[serde(rename = "kafka_bootstrap_brokers")]
        bootstrap_brokers: String,

        /// The consumer group ID, used to track the stream offset between restarts
        /// (due to host maintenance, upgrades, crashes, etc.).
        #[serde(rename = "kafka_group_id")]
        group_id: String,

        /// The topic to listen to.
        #[serde(rename = "kafka_topic")]
        topic: String,

        /// The value for 'security.protocol' in the kafka config.
        #[serde(flatten)]
        security_protocol: KafkaSecurityProtocol,

        /// The 'debug' config value for rdkafka - enables more verbose logging
        /// for the selected 'contexts'
        #[serde(rename = "kafka_debug_contexts")]
        debug_contexts: Option<String>,
    },
}

impl KafkaInputOpts {
    pub(crate) fn create_consumer(self) -> KafkaResult<StreamConsumer> {
        let Self::Inner {
            bootstrap_brokers,
            group_id,
            security_protocol,
            debug_contexts,
            ..
        } = self;

        let mut config = ClientConfig::new();
        config
            .set("group.id", group_id)
            .set("bootstrap.servers", bootstrap_brokers)
            // messages are committed manually after webhook delivery was successful.
            .set("enable.auto.commit", "false");

        security_protocol.apply(&mut config);
        if let Some(debug_contexts) = debug_contexts {
            if !debug_contexts.is_empty() {
                config.set("debug", debug_contexts);
            }
        }

        config.create()
    }
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type")]
pub enum KafkaOutputOpts {
    // Single-variant enum so we can require the "type": "kafka" field in deserialization
    #[serde(rename = "kafka")]
    Inner {
        /// Comma-separated list of addresses.
        ///
        /// Example: `localhost:9094`
        #[serde(rename = "kafka_bootstrap_brokers")]
        bootstrap_brokers: String,

        /// The topic to listen to.
        #[serde(rename = "kafka_topic")]
        topic: String,

        /// The value for 'security.protocol' in the kafka config.
        #[serde(flatten)]
        security_protocol: KafkaSecurityProtocol,

        /// The 'debug' config value for rdkafka - enables more verbose logging
        /// for the selected 'contexts'
        #[serde(rename = "kafka_debug_contexts")]
        debug_contexts: Option<String>,
    },
}

impl KafkaOutputOpts {
    pub(crate) fn create_producer(self) -> KafkaResult<FutureProducer> {
        let Self::Inner {
            bootstrap_brokers,
            security_protocol,
            debug_contexts,
            ..
        } = self;

        let mut config = ClientConfig::new();
        config.set("bootstrap.servers", bootstrap_brokers);

        security_protocol.apply(&mut config);
        if let Some(debug_contexts) = debug_contexts {
            if !debug_contexts.is_empty() {
                config.set("debug", debug_contexts);
            }
        }

        config.create()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kafka_security_protocol", rename_all = "snake_case")]
pub enum KafkaSecurityProtocol {
    Plaintext,
    Ssl,
    SaslSsl {
        #[serde(rename = "kafka_sasl_username")]
        sasl_username: String,
        #[serde(rename = "kafka_sasl_password")]
        sasl_password: String,
    },
}

impl KafkaSecurityProtocol {
    fn apply(self, config: &mut ClientConfig) {
        match self {
            KafkaSecurityProtocol::Plaintext => {
                config.set("security.protocol", "plaintext");
            }
            KafkaSecurityProtocol::Ssl => {
                config.set("security.protocol", "ssl");
            }
            KafkaSecurityProtocol::SaslSsl {
                sasl_username,
                sasl_password,
            } => {
                config
                    .set("security.protocol", "sasl_ssl")
                    .set("sasl.mechanisms", "SCRAM-SHA-512")
                    .set("sasl.username", sasl_username)
                    .set("sasl.password", sasl_password);
            }
        }
    }
}

pub fn into_sender_input(
    name: String,
    opts: KafkaInputOpts,
    transformation: Option<TransformationConfig>,
    output: SenderOutputOpts,
) -> Result<Box<dyn SenderInput>> {
    Ok(Box::new(KafkaConsumer::new(
        name,
        opts,
        transformation,
        output,
    )?))
}

pub fn into_receiver_output(
    name: String,
    opts: KafkaOutputOpts,
) -> Result<Box<dyn ReceiverOutput>> {
    Ok(Box::new(KafkaProducer::new(name, opts)?))
}
