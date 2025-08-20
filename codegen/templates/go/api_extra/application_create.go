// Get or create a new application.
func (application *Application) GetOrCreate(
	ctx context.Context,
	applicationIn models.ApplicationIn,
	o *ApplicationCreateOptions,
) (*models.ApplicationOut, error) {
	queryMap := map[string]string{
		"get_if_exists": "true",
	}
	headerMap := map[string]string{}

	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return internal.ExecuteRequest[models.ApplicationIn, models.ApplicationOut](
		ctx,
		application.client,
		"POST",
		"/api/v1/app",
		nil,
		queryMap,
		headerMap,
		&applicationIn,
	)
}
