// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type IngestAuthentication struct {
	client *internal.SvixHttpClient
}

func newIngestAuthentication(client *internal.SvixHttpClient) IngestAuthentication {
	return IngestAuthentication{client}
}

type IngestAuthenticationConsumerPortalAccessOptions struct {
	IdempotencyKey *string
}

// Get access to the Ingest Source Consumer Portal.
func (ingestAuthentication IngestAuthentication) ConsumerPortalAccess(
	ctx context.Context,
	sourceId string,
	ingestSourceConsumerPortalAccessIn models.IngestSourceConsumerPortalAccessIn,
	o *IngestAuthenticationConsumerPortalAccessOptions,
) (*models.AppPortalAccessOut, error) {
	var err error
	pathMap := map[string]string{
		"source_id": sourceId,
	}
	headerMap := map[string]string{}
	if o == nil {
		opts := IngestAuthenticationConsumerPortalAccessOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[models.IngestSourceConsumerPortalAccessIn, models.AppPortalAccessOut](
		ctx,
		ingestAuthentication.client,
		"POST",
		"/ingest/api/v1/source/{source_id}/dashboard",
		pathMap,
		nil,
		headerMap,
		&ingestSourceConsumerPortalAccessIn,
	)
}
