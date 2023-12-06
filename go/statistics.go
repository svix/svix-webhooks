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
	req := s.api.StatisticsApi.V1StatisticsAggregateAppStats(ctx)
	if appUsageStatsIn != nil {
		req = req.AppUsageStatsIn(*appUsageStatsIn)
	}
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := AppUsageStatsOut(out)
	return &ret, nil
}

func (s *Statistics) AggregateEventTypes(ctx context.Context) (*AggregateEventTypesOut, error) {
	req := s.api.StatisticsApi.V1StatisticsAggregateEventTypes(ctx)

	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := AggregateEventTypesOut(out)
	return &ret, nil
}
