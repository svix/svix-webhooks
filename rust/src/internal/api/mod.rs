pub(crate) mod endpoint_auto_config;

use std::sync::Arc;

pub(crate) use endpoint_auto_config::EndpointAutoConfig;

use crate::Configuration;

pub(crate) struct InternalApi {
    pub(super) cfg: Arc<Configuration>,
}

impl InternalApi {
    pub fn new(cfg: Arc<Configuration>) -> Self {
        Self { cfg }
    }

    pub fn auto_config(&self) -> EndpointAutoConfig<'_> {
        EndpointAutoConfig::new(&self.cfg)
    }
}
