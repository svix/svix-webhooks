// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AzureBlobStorageConfigOut {
    pub container: String,

    pub account: String,
}
