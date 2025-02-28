use super::IngestEndpoint;
use crate::Configuration;

pub struct Ingest<'a> {
    cfg: &'a Configuration,
}

impl<'a> Ingest<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn endpoint(&self) -> IngestEndpoint<'a> {
        IngestEndpoint::new(self.cfg)
    }
}
