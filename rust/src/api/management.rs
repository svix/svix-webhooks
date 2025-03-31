use super::ManagementAuthentication;
use crate::Configuration;

pub struct Management<'a> {
    cfg: &'a Configuration,
}

impl<'a> Management<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn authentication(&self) -> ManagementAuthentication<'a> {
        ManagementAuthentication::new(self.cfg)
    }
}
