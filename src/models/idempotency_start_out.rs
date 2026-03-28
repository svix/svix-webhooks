// this file is @generated
use serde::{Deserialize, Serialize};

use super::idempotency_completed::IdempotencyCompleted;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "status", content = "data")]
pub enum IdempotencyStartOut {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "completed")]
    Completed(IdempotencyCompleted),
}
