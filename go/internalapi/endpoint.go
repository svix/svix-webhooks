// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type Endpoint struct {
	MagicSubscription *EndpointMagicSubscription
}

func newEndpoint(client *internal.SvixHttpClient) *Endpoint {
	return &Endpoint{
		MagicSubscription: newEndpointMagicSubscription(client),
	}
}
