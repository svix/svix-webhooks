// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct AdminAuthRole<'a> {
    cfg: &'a Configuration,
}

impl<'a> AdminAuthRole<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create or update a role
    pub async fn upsert(
        &self,
        admin_role_upsert_in: AdminRoleUpsertIn,
    ) -> Result<AdminRoleUpsertOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-role.upsert")
            .with_body(admin_role_upsert_in)
            .execute(self.cfg)
            .await
    }

    /// Delete a role
    pub async fn delete(
        &self,
        admin_role_delete_in: AdminRoleDeleteIn,
    ) -> Result<AdminRoleDeleteOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-role.delete")
            .with_body(admin_role_delete_in)
            .execute(self.cfg)
            .await
    }

    /// Get a role by ID
    pub async fn get(&self, admin_role_get_in: AdminRoleGetIn) -> Result<AdminRoleOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-role.get")
            .with_body(admin_role_get_in)
            .execute(self.cfg)
            .await
    }

    /// List all roles
    pub async fn list(
        &self,
        admin_role_list_in: AdminRoleListIn,
    ) -> Result<ListResponseAdminRoleOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-role.list")
            .with_body(admin_role_list_in)
            .execute(self.cfg)
            .await
    }
}
