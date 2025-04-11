/** @deprecated Please use appPortalAccess instead. */
@Deprecated
public com.svix.models.DashboardAccessOut dashboardAccess(final String appId)
        throws IOException, ApiException {
    return this.dashboardAccess(appId, new AuthenticationDashboardAccessOptions());
}

/** @deprecated Please use appPortalAccess instead. */
@Deprecated
public com.svix.models.DashboardAccessOut dashboardAccess(
        final String appId, final AuthenticationDashboardAccessOptions options)
        throws IOException, ApiException {
    HttpUrl.Builder url =
            this.client
                    .newUrlBuilder()
                    .encodedPath(String.format("/api/v1/auth/dashboard-access/%s", appId));
    Map<String, String> headers = new HashMap<>();
    if (options.idempotencyKey != null) {
        headers.put("idempotency-key", options.idempotencyKey);
    }
    return this.client.executeRequest(
            "POST", url.build(), Headers.of(headers), null, com.svix.models.DashboardAccessOut.class);
}
