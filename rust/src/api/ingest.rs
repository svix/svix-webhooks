// this file is @generated
use super::{IngestAuthentication, IngestEndpoint, IngestSource};
use crate::Configuration;

pub struct Ingest<'a> {
    cfg: &'a Configuration,
}

impl<'a> Ingest<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn authentication(&self) -> IngestAuthentication<'a> {
        IngestAuthentication::new(self.cfg)
    }

    pub fn endpoint(&self) -> IngestEndpoint<'a> {
        IngestEndpoint::new(self.cfg)
    }

    pub fn source(&self) -> IngestSource<'a> {
        IngestSource::new(self.cfg)
    }
}
