// this file is @generated
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type Statistics struct {
	api *openapi.APIClient
}

// Creates a background task to calculate the message destinations for all applications in the environment.
//
// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
// retrieve the results of the operation.
func (statistics *Statistics) AggregateAppStats(
	ctx context.Context,
	appUsageStatsIn *AppUsageStatsIn,
) (*AppUsageStatsOut, error) {
	return statistics.AggregateAppStatsWithOptions(
		ctx,
		appUsageStatsIn,
		nil,
	)
}

// Creates a background task to calculate the message destinations for all applications in the environment.
//
// Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
// retrieve the results of the operation.
func (statistics *Statistics) AggregateAppStatsWithOptions(
	ctx context.Context,
	appUsageStatsIn *AppUsageStatsIn,
	options *PostOptions,
) (*AppUsageStatsOut, error) {
	req := statistics.api.StatisticsAPI.V1StatisticsAggregateAppStats(
		ctx,
	).AppUsageStatsIn(*appUsageStatsIn)

	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
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
	req := statistics.api.StatisticsAPI.V1StatisticsAggregateEventTypes(
		ctx,
	)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}
