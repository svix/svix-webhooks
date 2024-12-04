use std::str;

use rdkafka::error::KafkaError;
use svix_bridge_types::svix::error::Error as SvixClientError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("kafka error: {0}")]
    Kafka(#[from] KafkaError),

    #[error("svix client error")]
    SvixClient(#[from] SvixClientError),

    #[error("JSON deserialization failed")]
    Deserialization(#[source] serde_json::Error),

    #[error("non-UTF8 payload")]
    NonUtf8Payload(#[source] str::Utf8Error),

    #[error("kafka message is missing payload")]
    MissingPayload,

    #[error("transformation error: {error}")]
    Transformation { error: String },
}

impl Error {
    pub(crate) fn transformation(error: impl Into<String>) -> Self {
        Self::Transformation {
            error: error.into(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
