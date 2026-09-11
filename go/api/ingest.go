// Package svix this file is @generated DO NOT EDIT
package api

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type Ingest struct {
	client *internal.SvixHttpClient
}

func NewIngest(client *internal.SvixHttpClient) Ingest {
	return Ingest{client}
}

func (ingest Ingest) Authentication() IngestAuthentication {
	return NewIngestAuthentication(ingest.client)
}
func (ingest Ingest) Endpoint() IngestEndpoint {
	return NewIngestEndpoint(ingest.client)
}
func (ingest Ingest) Source() IngestSource {
	return NewIngestSource(ingest.client)
}
