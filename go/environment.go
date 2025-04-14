// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type Environment struct {
	client *internal.SvixHttpClient
}

func newEnvironment(client *internal.SvixHttpClient) *Environment {
	return &Environment{
		client: client,
	}
}

type EnvironmentExportOptions struct {
	IdempotencyKey *string
}

type EnvironmentImportOptions struct {
	IdempotencyKey *string
}

// Download a JSON file containing all org-settings and event types.
func (environment *Environment) Export(
	ctx context.Context,
	o *EnvironmentExportOptions,
) (*models.EnvironmentOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[any, models.EnvironmentOut](
		ctx,
		environment.client,
		"POST",
		"/api/v1/environment/export",
		nil,
		nil,
		headerMap,
		nil,
	)
}

// Import a configuration into the active organization.
//
// It doesn't delete anything, only adds / updates what was passed to it.
func (environment *Environment) Import(
	ctx context.Context,
	environmentIn models.EnvironmentIn,
	o *EnvironmentImportOptions,
) error {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}
	_, err = internal.ExecuteRequest[models.EnvironmentIn, any](
		ctx,
		environment.client,
		"POST",
		"/api/v1/environment/import",
		nil,
		nil,
		headerMap,
		&environmentIn,
	)
	return err
}
