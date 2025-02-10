// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/svix/svix-webhooks/go/models"
)

type Application struct {
	_client *SvixHttpClient
}

type ApplicationListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type ApplicationCreateOptions struct {
	IdempotencyKey *string
}

// List of all the organization's applications.
func (application *Application) List(
	ctx context.Context,
	o *ApplicationListOptions,
) (*models.ListResponseApplicationOut, error) {
	pathMap := map[string]string{}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("limit", o.Limit, queryMap, &err)
		SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		SerializeParamToMap("order", o.Order, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, apiErr := executeRequest[models.ListResponseApplicationOut](
		ctx,
		application._client,
		"GET",
		"/api/v1/app",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
}

// Create a new application.
func (application *Application) Create(
	ctx context.Context,
	applicationIn *models.ApplicationIn,
	o *ApplicationCreateOptions,
) (*models.ApplicationOut, error) {
	if applicationIn == nil {
		return nil, fmt.Errorf("Application.Create(), applicationIn must not be nil")
	}
	pathMap := map[string]string{}
	queryMap := map[string]string{
		"get_if_exists": "false",
	}
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	jsonBody, err := json.Marshal(applicationIn)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[models.ApplicationOut](
		ctx,
		application._client,
		"POST",
		"/api/v1/app",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
}

// Get or create a new application.
func (application *Application) GetOrCreate(
	ctx context.Context,
	applicationIn *models.ApplicationIn,
	o *ApplicationCreateOptions,
) (*models.ApplicationOut, error) {
	if applicationIn == nil {
		return nil, fmt.Errorf("Application.GetOrCreate(), applicationIn must not be nil")
	}
	pathMap := map[string]string{}
	queryMap := map[string]string{
		"get_if_exists": "true",
	}
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	jsonBody, err := json.Marshal(applicationIn)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[models.ApplicationOut](
		ctx,
		application._client,
		"POST",
		"/api/v1/app",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
}

// Get an application.
func (application *Application) Get(
	ctx context.Context,
	appId string,
) (*models.ApplicationOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, apiErr := executeRequest[models.ApplicationOut](
		ctx,
		application._client,
		"GET",
		"/api/v1/app/{app_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
}

// Update an application.
func (application *Application) Update(
	ctx context.Context,
	appId string,
	applicationIn *models.ApplicationIn,
) (*models.ApplicationOut, error) {
	if applicationIn == nil {
		return nil, fmt.Errorf("Application.Update(), applicationIn must not be nil")
	}
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(applicationIn)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[models.ApplicationOut](
		ctx,
		application._client,
		"PUT",
		"/api/v1/app/{app_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
}

// Delete an application.
func (application *Application) Delete(
	ctx context.Context,
	appId string,
) error {
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	_, apiErr := executeRequest[any](
		ctx,
		application._client,
		"DELETE",
		"/api/v1/app/{app_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return apiErr
	}
	return nil
}

// Partially update an application.
func (application *Application) Patch(
	ctx context.Context,
	appId string,
	applicationPatch *models.ApplicationPatch,
) (*models.ApplicationOut, error) {
	if applicationPatch == nil {
		return nil, fmt.Errorf("Application.Patch(), applicationPatch must not be nil")
	}
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(applicationPatch)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[models.ApplicationOut](
		ctx,
		application._client,
		"PATCH",
		"/api/v1/app/{app_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if apiErr != nil {
		return nil, apiErr
	}
	return ret, nil
}
