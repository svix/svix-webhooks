// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/svix/svix-webhooks/go/models"
)

type EventType struct {
	client *SvixHttpClient
}

type EventTypeListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
	// When `true` archived (deleted but not expunged) items are included in the response.
	IncludeArchived *bool
	// When `true` the full item (including the schema) is included in the response.
	WithContent *bool
}

type EventTypeCreateOptions struct {
	IdempotencyKey *string
}

type EventTypeImportOpenapiOptions struct {
	IdempotencyKey *string
}

type EventTypeDeleteOptions struct {
	// By default event types are archived when "deleted". Passing this to `true` deletes them entirely.
	Expunge *bool
}

// Return the list of event types.
func (eventType *EventType) List(
	ctx context.Context,
	o *EventTypeListOptions,
) (*models.ListResponseEventTypeOut, error) {
	pathMap := map[string]string{}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	var err error
	if o != nil {
		SerializeParamToMap("limit", o.Limit, queryMap, &err)
		SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		SerializeParamToMap("order", o.Order, queryMap, &err)
		SerializeParamToMap("include_archived", o.IncludeArchived, queryMap, &err)
		SerializeParamToMap("with_content", o.WithContent, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, err := executeRequest[models.ListResponseEventTypeOut](
		ctx,
		eventType.client,
		"GET",
		"/api/v1/event-type",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Create new or unarchive existing event type.
//
// Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
// Endpoints filtering on the event type before archival will continue to filter on it.
// This operation does not preserve the description and schemas.
func (eventType *EventType) Create(
	ctx context.Context,
	eventTypeIn *models.EventTypeIn,
	o *EventTypeCreateOptions,
) (*models.EventTypeOut, error) {
	if eventTypeIn == nil {
		return nil, fmt.Errorf("EventType.Create(), eventTypeIn must not be nil")
	}
	pathMap := map[string]string{}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	var err error
	if o != nil {
		SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	jsonBody, err = json.Marshal(eventTypeIn)
	if err != nil {
		return nil, err
	}
	ret, err := executeRequest[models.EventTypeOut](
		ctx,
		eventType.client,
		"POST",
		"/api/v1/event-type",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Given an OpenAPI spec, create new or update existing event types.
// If an existing `archived` event type is updated, it will be unarchived.
//
// The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
// top-level.
func (eventType *EventType) ImportOpenapi(
	ctx context.Context,
	eventTypeImportOpenApiIn *models.EventTypeImportOpenApiIn,
	o *EventTypeImportOpenapiOptions,
) (*models.EventTypeImportOpenApiOut, error) {
	if eventTypeImportOpenApiIn == nil {
		return nil, fmt.Errorf("EventType.ImportOpenapi(), eventTypeImportOpenApiIn must not be nil")
	}
	pathMap := map[string]string{}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	var err error
	if o != nil {
		SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	jsonBody, err = json.Marshal(eventTypeImportOpenApiIn)
	if err != nil {
		return nil, err
	}
	ret, err := executeRequest[models.EventTypeImportOpenApiOut](
		ctx,
		eventType.client,
		"POST",
		"/api/v1/event-type/import/openapi",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Get an event type.
func (eventType *EventType) Get(
	ctx context.Context,
	eventTypeName string,
) (*models.EventTypeOut, error) {
	pathMap := map[string]string{
		"event_type_name": eventTypeName,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, err := executeRequest[models.EventTypeOut](
		ctx,
		eventType.client,
		"GET",
		"/api/v1/event-type/{event_type_name}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Update an event type.
func (eventType *EventType) Update(
	ctx context.Context,
	eventTypeName string,
	eventTypeUpdate *models.EventTypeUpdate,
) (*models.EventTypeOut, error) {
	if eventTypeUpdate == nil {
		return nil, fmt.Errorf("EventType.Update(), eventTypeUpdate must not be nil")
	}
	pathMap := map[string]string{
		"event_type_name": eventTypeName,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(eventTypeUpdate)
	if err != nil {
		return nil, err
	}
	ret, err := executeRequest[models.EventTypeOut](
		ctx,
		eventType.client,
		"PUT",
		"/api/v1/event-type/{event_type_name}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Archive an event type.
//
// Endpoints already configured to filter on an event type will continue to do so after archival.
// However, new messages can not be sent with it and endpoints can not filter on it.
// An event type can be unarchived with the
// [create operation](#operation/create_event_type_api_v1_event_type__post).
func (eventType *EventType) Delete(
	ctx context.Context,
	eventTypeName string,
	o *EventTypeDeleteOptions,
) error {
	pathMap := map[string]string{
		"event_type_name": eventTypeName,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	var err error
	if o != nil {
		SerializeParamToMap("expunge", o.Expunge, queryMap, &err)
		if err != nil {
			return err
		}
	}
	_, err = executeRequest[any](
		ctx,
		eventType.client,
		"DELETE",
		"/api/v1/event-type/{event_type_name}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return err
	}
	return nil
}

// Partially update an event type.
func (eventType *EventType) Patch(
	ctx context.Context,
	eventTypeName string,
	eventTypePatch *models.EventTypePatch,
) (*models.EventTypeOut, error) {
	if eventTypePatch == nil {
		return nil, fmt.Errorf("EventType.Patch(), eventTypePatch must not be nil")
	}
	pathMap := map[string]string{
		"event_type_name": eventTypeName,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	jsonBody, err := json.Marshal(eventTypePatch)
	if err != nil {
		return nil, err
	}
	ret, err := executeRequest[models.EventTypeOut](
		ctx,
		eventType.client,
		"PATCH",
		"/api/v1/event-type/{event_type_name}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}
