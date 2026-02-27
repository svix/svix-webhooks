// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PingOut {
    pub ok: bool,
}

impl PingOut {
    pub fn new(ok: bool) -> Self {
        Self { ok }
    }
}
