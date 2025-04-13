// Deprecated: Please use `AppPortalAccess` instead.
func (authentication *Authentication) DashboardAccess(
	ctx context.Context,
	appId string,
	o *AuthenticationDashboardAccessOptions,
) (*models.DashboardAccessOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return executeRequest[any, models.DashboardAccessOut](
		ctx,
		authentication.client,
		"POST",
		"/api/v1/auth/dashboard-access/{app_id}",
		pathMap,
		nil,
		headerMap,
		nil,
	)
}
