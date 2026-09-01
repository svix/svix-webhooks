// this file is @generated
use super::DestinationAutoconfig;
use crate::Configuration;

pub struct Destination<'a> {
    cfg: &'a Configuration,
}

impl<'a> Destination<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn autoconfig(&self) -> DestinationAutoconfig<'a> {
        DestinationAutoconfig::new(self.cfg)
    }
}
