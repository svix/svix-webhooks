pub(crate) mod endpoint_auto_config;

pub(crate) use endpoint_auto_config::EndpointAutoConfig;

use crate::Configuration;

pub(crate) struct InternalApi<'a> {
    pub(super) cfg: &'a Configuration,
}

impl<'a> InternalApi<'a> {
    pub fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn auto_config(&self) -> EndpointAutoConfig<'_> {
        EndpointAutoConfig::new(self.cfg)
    }
}
