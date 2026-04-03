// this file is @generated
use super::{AdminAuthPolicy, AdminAuthRole, AdminAuthToken, AdminCluster};
use crate::Configuration;

pub struct Admin<'a> {
    cfg: &'a Configuration,
}

impl<'a> Admin<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn auth_policy(&self) -> AdminAuthPolicy<'a> {
        AdminAuthPolicy::new(self.cfg)
    }

    pub fn auth_role(&self) -> AdminAuthRole<'a> {
        AdminAuthRole::new(self.cfg)
    }

    pub fn auth_token(&self) -> AdminAuthToken<'a> {
        AdminAuthToken::new(self.cfg)
    }

    pub fn cluster(&self) -> AdminCluster<'a> {
        AdminCluster::new(self.cfg)
    }
}
