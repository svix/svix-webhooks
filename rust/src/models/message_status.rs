// this file is @generated
use std::fmt;

use serde_repr::{
    Deserialize_repr,
    Serialize_repr,
};

/// The sending status of the message:
///
/// - Success = 0
/// - Pending = 1
/// - Fail = 2
/// - Sending = 3
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
pub enum MessageStatus {
    #[default]
    Success = 0,
    Pending = 1,
    Fail = 2,
    Sending = 3,
}

impl fmt::Display for MessageStatus {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(
            f,
            "{}",
            *self as i64
        )
    }
}
