#[deprecated = "Please use app_portal_access instead."]
#[allow(deprecated)]
pub async fn dashboard_access(
    &self,
    app_id: String,
    options: Option<super::AuthenticationDashboardAccessOptions>,
) -> Result<DashboardAccessOut> {
    let super::AuthenticationDashboardAccessOptions { idempotency_key } =
        options.unwrap_or_default();

    crate::request::Request::new(
        http1::Method::POST,
        "/api/v1/auth/dashboard-access/{app_id}",
    )
    .with_path_param("app_id", app_id)
    .with_optional_header_param("idempotency-key", idempotency_key)
    .execute(self.cfg)
    .await
}
