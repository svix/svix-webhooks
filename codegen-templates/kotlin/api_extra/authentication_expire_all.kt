/** @deprecated Please use `appPortalAccess` instead. */
@Deprecated("Please use `appPortalAccess` instead.")
suspend fun dashboardAccess(
    appId: String,
    options: AuthenticationDashboardAccessOptions = AuthenticationDashboardAccessOptions(),
): com.svix.kotlin.models.DashboardAccessOut {
    val url = client.newUrlBuilder().encodedPath("/api/v1/auth/dashboard-access/$appId")
    val headers = Headers.Builder()
    options.idempotencyKey?.let { headers.add("idempotency-key", it) }
    return client.executeRequest<Any, com.svix.kotlin.models.DashboardAccessOut>(
        "POST",
        url.build(),
        headers = headers.build(),
    )
}
