// this file is @generated
use super::AdminCluster;
use crate::Configuration;

pub struct Admin<'a> {
    cfg: &'a Configuration,
}

impl<'a> Admin<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn cluster(&self) -> AdminCluster<'a> {
        AdminCluster::new(self.cfg)
    }
}
