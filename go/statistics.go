// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type Statistics struct {
	client *internal.SvixHttpClient
}

func newStatistics(client *internal.SvixHttpClient) *Statistics {
	return &Statistics{
		client: client,
	}
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
	appUsageStatsIn models.AppUsageStatsIn,
	o *StatisticsAggregateAppStatsOptions,
) (*models.AppUsageStatsOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.AppUsageStatsIn, models.AppUsageStatsOut](
		ctx,
		statistics.client,
		"POST",
		"/api/v1/stats/usage/app",
		nil,
		nil,
		headerMap,
		&appUsageStatsIn,
	)
}

// Creates a background task to calculate the listed event types for all apps in the organization.
//
// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
// retrieve the results of the operation.
func (statistics *Statistics) AggregateEventTypes(
	ctx context.Context,
) (*models.AggregateEventTypesOut, error) {
	return internal.ExecuteRequest[any, models.AggregateEventTypesOut](
		ctx,
		statistics.client,
		"PUT",
		"/api/v1/stats/usage/event-types",
		nil,
		nil,
		nil,
		nil,
	)
}
