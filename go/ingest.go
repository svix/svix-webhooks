// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type Ingest struct {
	client *internal.SvixHttpClient
}

func newIngest(client *internal.SvixHttpClient) Ingest {
	return Ingest{client}
}

func (ingest Ingest) Authentication() IngestAuthentication {
	return newIngestAuthentication(ingest.client)
}
func (ingest Ingest) Endpoint() IngestEndpoint {
	return newIngestEndpoint(ingest.client)
}
func (ingest Ingest) Source() IngestSource {
	return newIngestSource(ingest.client)
}
