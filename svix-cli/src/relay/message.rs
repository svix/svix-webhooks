//! Similar to the type definitions in the relay server except In/Out nomenclature is reversed.
//!
//! `MessageIn` represents messages sent from relay to the CLI, which will then be forwarded to the
//! local server.
//! `MessageOut` represents the responses we get from the local server which will then be forwarded
//! back to the relay.
//!
//! The main difference between the two is the `-In` has an HTTP method on it (needed so we can
//! recreate the request properly to the local server, whereas the `-Out` has a status code.

use std::collections::HashMap;

use serde::{
    Deserialize,
    Serialize,
};

pub const VERSION: u16 = 1;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageOutStart {
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageOutEvent {
    pub id: String,
    pub status: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MessageOut {
    Start { version: u16, data: MessageOutStart },
    Event { version: u16, data: MessageOutEvent },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageInStart {
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageInEvent {
    pub id: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub method: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MessageIn {
    Start { version: u16, data: MessageInStart },
    Event { version: u16, data: MessageInEvent },
}
