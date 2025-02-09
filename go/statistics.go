// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"encoding/json"
	"fmt"
)

type Statistics struct {
	_client *SvixHttpClient
}

type StatisticsAggregateAppStatsOptions struct {
	IdempotencyKey *string
}

// Creates a background task to calculate the message destinations for all applications in the environment.
//
// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
// retrieve the results of the operation.
func (statistics *Statistics) AggregateAppStats(
	ctx context.Context,
	appUsageStatsIn *AppUsageStatsIn,
	o *StatisticsAggregateAppStatsOptions,
) (*AppUsageStatsOut, error) {
	if appUsageStatsIn == nil {
		return nil, fmt.Errorf("Statistics.AggregateAppStats(), appUsageStatsIn must not be nil")
	}
	pathMap := map[string]string{}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	if o != nil {
		var err error
		SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	jsonBody, err := json.Marshal(appUsageStatsIn)
	if err != nil {
		return nil, err
	}
	ret, apiErr := executeRequest[AppUsageStatsOut](
		ctx,
		statistics._client,
		"POST",
		"/api/v1/stats/usage/app",
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

// Creates a background task to calculate the listed event types for all apps in the organization.
//
// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
// retrieve the results of the operation.
func (statistics *Statistics) AggregateEventTypes(
	ctx context.Context,
) (*AggregateEventTypesOut, error) {
	pathMap := map[string]string{}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, apiErr := executeRequest[AggregateEventTypesOut](
		ctx,
		statistics._client,
		"PUT",
		"/api/v1/stats/usage/event-types",
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
