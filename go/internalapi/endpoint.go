// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type Endpoint struct {
	client *internal.SvixHttpClient
}

func newEndpoint(client *internal.SvixHttpClient) Endpoint {
	return Endpoint{client}
}

func (endpoint Endpoint) AutoConfig() EndpointAutoConfig {
	return newEndpointAutoConfig(endpoint.client)
}
