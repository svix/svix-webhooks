// this file is @generated
use serde::{Deserialize, Serialize};

use super::batch_message_in::BatchMessageIn;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BatchCreateMessagesIn {
    pub messages: Vec<BatchMessageIn>,
}

impl BatchCreateMessagesIn {
    pub fn new(messages: Vec<BatchMessageIn>) -> Self {
        Self { messages }
    }
}
