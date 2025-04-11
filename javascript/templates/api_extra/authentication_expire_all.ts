/** @deprecated Please use `appPortalAccess` instead. */
  public dashboardAccess(
  appId: string,
  options?: AuthenticationDashboardAccessOptions
): Promise<DashboardAccessOut> {
  const request = new SvixRequest(
    HttpMethod.POST,
    "/api/v1/auth/dashboard-access/{app_id}"
  );

  request.setPathParam("app_id", appId);
  request.setHeaderParam("idempotency-key", options?.idempotencyKey);

  return request.send(this.requestCtx, DashboardAccessOutSerializer._fromJsonObject);
}
