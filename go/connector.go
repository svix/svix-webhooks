// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal"
	"github.com/svix/svix-webhooks/go/models"
)

type Connector struct {
	client *internal.SvixHttpClient
}

func newConnector(client *internal.SvixHttpClient) *Connector {
	return &Connector{
		client: client,
	}
}

type ConnectorListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type ConnectorCreateOptions struct {
	IdempotencyKey *string
}

// List all connectors for an application.
func (connector *Connector) List(
	ctx context.Context,
	o *ConnectorListOptions,
) (*models.ListResponseConnectorOut, error) {
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
	return internal.ExecuteRequest[any, models.ListResponseConnectorOut](
		ctx,
		connector.client,
		"GET",
		"/api/v1/connector",
		nil,
		queryMap,
		nil,
		nil,
	)
}

// Create a new connector.
func (connector *Connector) Create(
	ctx context.Context,
	connectorIn models.ConnectorIn,
	o *ConnectorCreateOptions,
) (*models.ConnectorOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return internal.ExecuteRequest[models.ConnectorIn, models.ConnectorOut](
		ctx,
		connector.client,
		"POST",
		"/api/v1/connector",
		nil,
		nil,
		headerMap,
		&connectorIn,
	)
}

// Get a connector.
func (connector *Connector) Get(
	ctx context.Context,
	connectorId string,
) (*models.ConnectorOut, error) {
	pathMap := map[string]string{
		"connector_id": connectorId,
	}
	return internal.ExecuteRequest[any, models.ConnectorOut](
		ctx,
		connector.client,
		"GET",
		"/api/v1/connector/{connector_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
}

// Update a connector.
func (connector *Connector) Update(
	ctx context.Context,
	connectorId string,
	connectorUpdate models.ConnectorUpdate,
) (*models.ConnectorOut, error) {
	pathMap := map[string]string{
		"connector_id": connectorId,
	}
	return internal.ExecuteRequest[models.ConnectorUpdate, models.ConnectorOut](
		ctx,
		connector.client,
		"PUT",
		"/api/v1/connector/{connector_id}",
		pathMap,
		nil,
		nil,
		&connectorUpdate,
	)
}

// Delete a connector.
func (connector *Connector) Delete(
	ctx context.Context,
	connectorId string,
) error {
	pathMap := map[string]string{
		"connector_id": connectorId,
	}
	_, err := internal.ExecuteRequest[any, any](
		ctx,
		connector.client,
		"DELETE",
		"/api/v1/connector/{connector_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err
}

// Partially update a connector.
func (connector *Connector) Patch(
	ctx context.Context,
	connectorId string,
	connectorPatch models.ConnectorPatch,
) (*models.ConnectorOut, error) {
	pathMap := map[string]string{
		"connector_id": connectorId,
	}
	return internal.ExecuteRequest[models.ConnectorPatch, models.ConnectorOut](
		ctx,
		connector.client,
		"PATCH",
		"/api/v1/connector/{connector_id}",
		pathMap,
		nil,
		nil,
		&connectorPatch,
	)
}
