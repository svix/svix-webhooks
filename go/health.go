// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
)

type Health struct {
	client *internal.SvixHttpClient
}

func newHealth(client *internal.SvixHttpClient) *Health {
	return &Health{
		client: client,
	}
}

// Verify the API server is up and running.
func (health *Health) Get(
	ctx context.Context,
) error {
	var err error
	_, err = internal.ExecuteRequest[any, any](
		ctx,
		health.client,
		"GET",
		"/api/v1/health",
		nil,
		nil,
		nil,
		nil,
	)
	return err
}
