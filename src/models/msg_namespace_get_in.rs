// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct MsgNamespaceGetIn {}

impl MsgNamespaceGetIn {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgNamespaceGetIn_ {
    pub name: String,
}
