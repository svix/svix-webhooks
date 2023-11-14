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

func (s *Statistics) CalculateAggregateAppStats(ctx context.Context, appUsageStatsIn *AppUsageStatsIn, options *PostOptions) (*AppUsageStatsOut, error) {
	req := s.api.StatisticsApi.CalculateAggregateAppStats(ctx)
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
	req := s.api.StatisticsApi.AggregateEventTypes(ctx)

	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := AggregateEventTypesOut(out)
	return &ret, nil
}
