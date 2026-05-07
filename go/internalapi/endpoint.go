// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type Endpoint struct {
	AutoConfig *EndpointAutoConfig
}

func newEndpoint(client *internal.SvixHttpClient) *Endpoint {
	return &Endpoint{
		AutoConfig: newEndpointAutoConfig(client),
	}
}
