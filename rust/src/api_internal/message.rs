// this file is @generated
use super::MessagePollerv2;
use crate::Configuration;

pub struct Message<'a> {
    cfg: &'a Configuration,
}

impl<'a> Message<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn pollerv2(&self) -> MessagePollerv2<'a> {
        MessagePollerv2::new(self.cfg)
    }
}

#[cfg(feature = "svix_beta")]
#[derive(Clone, Debug)]
pub struct V1MessageEventsParams {
    /// The app's ID or UID
    pub app_id: String,
    /// Limit the number of returned items
    pub limit: Option<u64>,
    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,
    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>,
    /// Filter response based on the event type.
    pub channels: Option<Vec<String>>,
    pub after: Option<String>,
}

#[cfg(feature = "svix_beta")]
#[derive(Clone, Debug)]
pub struct V1MessageEventsSubscriptionParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The esub's ID or UID
    pub subscription_id: String,
    /// Limit the number of returned items
    pub limit: Option<u64>,
    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,
    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>,
    /// Filter response based on the event type.
    pub channels: Option<Vec<String>>,
    pub after: Option<String>,
}
