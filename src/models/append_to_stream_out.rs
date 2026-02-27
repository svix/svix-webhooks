// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppendToStreamOut {
    pub msg_ids: Vec<u64>,
}

impl AppendToStreamOut {
    pub fn new(msg_ids: Vec<u64>) -> Self {
        Self { msg_ids }
    }
}
