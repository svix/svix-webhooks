// this file is @generated
use std::fmt;

use serde_repr::{Deserialize_repr, Serialize_repr};

/// The reason an attempt was made:
///
/// - Scheduled = 0
/// - Manual = 1
#[repr(i64)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize_repr,
    Deserialize_repr,
)]
pub enum MessageAttemptTriggerType {
    #[default]
    Scheduled = 0,
    Manual = 1,
}

impl fmt::Display for MessageAttemptTriggerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as i64)
    }
}
