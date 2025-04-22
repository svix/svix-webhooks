@deprecated
def dashboard_access(
    self,
    app_id: str,
    options: AuthenticationDashboardAccessOptions = AuthenticationDashboardAccessOptions(),
) -> DashboardAccessOut:
    """Deprecated: Please use `app_portal_access` instead."""
    response = self._request_sync(
        method="post",
        path="/api/v1/auth/dashboard-access/{app_id}",
        path_params={
            "app_id": app_id,
        },
        query_params=options._query_params(),
        header_params=options._header_params(),
    )
    return DashboardAccessOut.model_validate(response.json())
