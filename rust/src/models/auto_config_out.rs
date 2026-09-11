// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AutoConfigOut {
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    pub token: String,

    /// The AutoConfigSubscription's ID.
    pub id: String,
}
