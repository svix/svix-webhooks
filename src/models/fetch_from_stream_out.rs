// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_out::MsgOut;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FetchFromStreamOut {
    pub msgs: Vec<MsgOut>,
}

impl FetchFromStreamOut {
    pub fn new(msgs: Vec<MsgOut>) -> Self {
        Self { msgs }
    }
}
