// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePrecheckOut {
    /// Whether there are any active endpoint that would get sent such a
    /// message.
    pub active: bool,
}

impl MessagePrecheckOut {
    pub fn new(active: bool) -> Self {
        Self { active }
    }
}
