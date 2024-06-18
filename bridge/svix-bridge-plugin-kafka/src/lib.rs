mod config;
mod error;
mod input;

pub use self::{
    config::{into_sender_input, KafkaInputOpts, KafkaSecurityProtocol},
    error::{Error, Result},
    input::KafkaConsumer,
};
