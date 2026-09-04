// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"github.com/svix/svix-webhooks/go/internal"
)

type Destination struct {
	client *internal.SvixHttpClient
}

func newDestination(client *internal.SvixHttpClient) Destination {
	return Destination{client}
}

func (destination Destination) Autoconfig() DestinationAutoconfig {
	return newDestinationAutoconfig(destination.client)
}
