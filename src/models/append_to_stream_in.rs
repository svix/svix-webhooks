// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_in::MsgIn;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppendToStreamIn {
    pub msgs: Vec<MsgIn>,

    pub name: String,
}

impl AppendToStreamIn {
    pub fn new(msgs: Vec<MsgIn>, name: String) -> Self {
        Self { msgs, name }
    }
}
