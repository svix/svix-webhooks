// Package svix this file is @generated DO NOT EDIT
package internalapi

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type ManagementEnvironment struct {
	client *internal.SvixHttpClient
}

func newManagementEnvironment(client *internal.SvixHttpClient) *ManagementEnvironment {
	return &ManagementEnvironment{
		client: client,
	}
}

type ManagementEnvironmentListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type ManagementEnvironmentCreateOptions struct {
	IdempotencyKey *string
}

// List all environments.
func (managementEnvironment *ManagementEnvironment) List(
	ctx context.Context,
	o *ManagementEnvironmentListOptions,
) (*models.ListResponseEnvironmentModelOut, error) {
	queryMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("limit", o.Limit, queryMap, &err)
		internal.SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		internal.SerializeParamToMap("order", o.Order, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[any, models.ListResponseEnvironmentModelOut](
		ctx,
		managementEnvironment.client,
		"GET",
		"/api/v1/management/environment",
		nil,
		queryMap,
		nil,
		nil,
	)
}

// Create a new environment.
func (managementEnvironment *ManagementEnvironment) Create(
	ctx context.Context,
	environmentModelIn models.EnvironmentModelIn,
	o *ManagementEnvironmentCreateOptions,
) (*models.EnvironmentModelOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.EnvironmentModelIn, models.EnvironmentModelOut](
		ctx,
		managementEnvironment.client,
		"POST",
		"/api/v1/management/environment",
		nil,
		nil,
		headerMap,
		&environmentModelIn,
	)
}

// Get an environment.
func (managementEnvironment *ManagementEnvironment) Get(
	ctx context.Context,
	envId string,
) (*models.EnvironmentModelOut, error) {
	pathMap := map[string]string{
		"env_id": envId,
	}
	return internal.ExecuteRequest[any, models.EnvironmentModelOut](
		ctx,
		managementEnvironment.client,
		"GET",
		"/api/v1/management/environment/{env_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Update an environment.
func (managementEnvironment *ManagementEnvironment) Update(
	ctx context.Context,
	envId string,
	environmentModelUpdate models.EnvironmentModelUpdate,
) (*models.EnvironmentModelOut, error) {
	pathMap := map[string]string{
		"env_id": envId,
	}
	return internal.ExecuteRequest[models.EnvironmentModelUpdate, models.EnvironmentModelOut](
		ctx,
		managementEnvironment.client,
		"PUT",
		"/api/v1/management/environment/{env_id}",
		pathMap,
		nil,
		nil,
		&environmentModelUpdate,
	)
}

// Delete an environment.
func (managementEnvironment *ManagementEnvironment) Delete(
	ctx context.Context,
	envId string,
) error {
	pathMap := map[string]string{
		"env_id": envId,
	}
	_, err := internal.ExecuteRequest[any, any](
		ctx,
		managementEnvironment.client,
		"DELETE",
		"/api/v1/management/environment/{env_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err
}
