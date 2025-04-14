// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type Ingest struct {
	client   *internal.SvixHttpClient
	Endpoint *IngestEndpoint
	Source   *IngestSource
}

func newIngest(client *internal.SvixHttpClient) *Ingest {
	return &Ingest{
		client:   client,
		Endpoint: newIngestEndpoint(client),
		Source:   newIngestSource(client),
	}
}

type IngestDashboardOptions struct {
	IdempotencyKey *string
}

// Get access to the Ingest Source Consumer Portal.
func (ingest *Ingest) Dashboard(
	ctx context.Context,
	sourceId string,
	ingestSourceConsumerPortalAccessIn models.IngestSourceConsumerPortalAccessIn,
	o *IngestDashboardOptions,
) (*models.DashboardAccessOut, error) {
	pathMap := map[string]string{
		"source_id": sourceId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.IngestSourceConsumerPortalAccessIn, models.DashboardAccessOut](
		ctx,
		ingest.client,
		"POST",
		"/ingest/api/v1/source/{source_id}/dashboard",
		pathMap,
		nil,
		headerMap,
		&ingestSourceConsumerPortalAccessIn,
	)
}
