// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedriveIn {
    pub consumer_group: String,

    pub name: String,
}

impl RedriveIn {
    pub fn new(consumer_group: String, name: String) -> Self {
        Self {
            consumer_group,
            name,
        }
    }
}
