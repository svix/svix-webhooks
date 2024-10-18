package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type Statistics struct {
	api *openapi.APIClient
}

type (
	AppUsageStatsIn        = openapi.AppUsageStatsIn
	AppUsageStatsOut       = openapi.AppUsageStatsOut
	AggregateEventTypesOut = openapi.AggregateEventTypesOut
)

func (s *Statistics) AggregateAppStats(ctx context.Context, appUsageStatsIn *AppUsageStatsIn, options *PostOptions) (*AppUsageStatsOut, error) {
	req := s.api.StatisticsAPI.V1StatisticsAggregateAppStats(ctx)
	if appUsageStatsIn != nil {
		req = req.AppUsageStatsIn(*appUsageStatsIn)
	}
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

func (s *Statistics) AggregateEventTypes(ctx context.Context) (*AggregateEventTypesOut, error) {
	req := s.api.StatisticsAPI.V1StatisticsAggregateEventTypes(ctx)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}
