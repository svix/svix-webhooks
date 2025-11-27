// this file is @generated
use super::OperationalWebhookEndpoint;
use crate::Configuration;

pub struct OperationalWebhook<'a> {
    cfg: &'a Configuration,
}

impl<'a> OperationalWebhook<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    pub fn endpoint(&self) -> OperationalWebhookEndpoint<'a> {
        OperationalWebhookEndpoint::new(self.cfg)
    }
}
