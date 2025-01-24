// this file is @generated (with some manual changes)
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type EventType struct {
	api *openapi.APIClient
}

type EventTypeListOptions struct {
	// Limit the number of returned items
	Limit *int32
	// The iterator returned from a prior invocation
	Iterator *string
	// The sorting order of the returned items
	Order *Ordering
	// When `true` archived (deleted but not expunged) items are included in the response.
	IncludeArchived *bool
	// When `true` the full item (including the schema) is included in the response.
	WithContent *bool
}

type EventTypeDeleteOptions struct {
	// By default event types are archived when "deleted". Passing this to `true` deletes them entirely.
	Expunge *bool
}

// Return the list of event types.
func (eventType *EventType) List(
	ctx context.Context,
	options *EventTypeListOptions,
) (*ListResponseEventTypeOut, error) {
	req := eventType.api.EventTypeAPI.V1EventTypeList(
		ctx,
	)

	if options != nil {
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Order != nil {
			req = req.Order(*options.Order)
		}
		if options.IncludeArchived != nil {
			req = req.IncludeArchived(*options.IncludeArchived)
		}
		if options.WithContent != nil {
			req = req.WithContent(*options.WithContent)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
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
	eventTypeIn *EventTypeIn,
) (*EventTypeOut, error) {
	return eventType.CreateWithOptions(
		ctx,
		eventTypeIn,
		nil,
	)
}

// Create new or unarchive existing event type.
//
// Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
// Endpoints filtering on the event type before archival will continue to filter on it.
// This operation does not preserve the description and schemas.
func (eventType *EventType) CreateWithOptions(
	ctx context.Context,
	eventTypeIn *EventTypeIn,
	options *PostOptions,
) (*EventTypeOut, error) {
	req := eventType.api.EventTypeAPI.V1EventTypeCreate(
		ctx,
	).EventTypeIn(*eventTypeIn)

	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
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
	eventTypeImportOpenApiIn *EventTypeImportOpenApiIn,
) (*EventTypeImportOpenApiOut, error) {
	return eventType.ImportOpenapiWithOptions(
		ctx,
		eventTypeImportOpenApiIn,
		nil,
	)
}

// Given an OpenAPI spec, create new or update existing event types.
// If an existing `archived` event type is updated, it will be unarchived.
//
// The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
// top-level.
func (eventType *EventType) ImportOpenapiWithOptions(
	ctx context.Context,
	eventTypeImportOpenApiIn *EventTypeImportOpenApiIn,
	options *PostOptions,
) (*EventTypeImportOpenApiOut, error) {
	req := eventType.api.EventTypeAPI.V1EventTypeImportOpenapi(
		ctx,
	).EventTypeImportOpenApiIn(*eventTypeImportOpenApiIn)

	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Deprecated: Use eventType.ImportOpenapi instead
func (eventType *EventType) ImportOpenApi(
	ctx context.Context,
	eventTypeImportOpenApiIn EventTypeImportOpenApiIn,
) (*EventTypeImportOpenApiOut, error) {
	return eventType.ImportOpenapi(ctx, &eventTypeImportOpenApiIn)
}

// Deprecated: Use eventType.ImportOpenapiWithOptions instead
func (eventType *EventType) ImportOpenApiWithOptions(
	ctx context.Context,
	eventTypeImportOpenApiIn EventTypeImportOpenApiIn,
	options *PostOptions,
) (*EventTypeImportOpenApiOut, error) {
	return eventType.ImportOpenapiWithOptions(ctx, &eventTypeImportOpenApiIn, options)
}

// Get an event type.
func (eventType *EventType) Get(
	ctx context.Context,
	eventTypeName string,
) (*EventTypeOut, error) {
	req := eventType.api.EventTypeAPI.V1EventTypeGet(
		ctx,
		eventTypeName,
	)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Update an event type.
func (eventType *EventType) Update(
	ctx context.Context,
	eventTypeName string,
	eventTypeUpdate *EventTypeUpdate,
) (*EventTypeOut, error) {
	req := eventType.api.EventTypeAPI.V1EventTypeUpdate(
		ctx,
		eventTypeName,
	).EventTypeUpdate(*eventTypeUpdate)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Archive an event type.
//
// Endpoints already configured to filter on an event type will continue to do so after archival.
// However, new messages can not be sent with it and endpoints can not filter on it.
// An event type can be unarchived with the create operation.
func (eventType *EventType) Delete(
	ctx context.Context,
	eventTypeName string,
) error {
	return eventType.DeleteWithOptions(ctx, eventTypeName, nil)
}

func (eventType *EventType) DeleteWithOptions(
	ctx context.Context,
	eventTypeName string,
	options *EventTypeDeleteOptions,
) error {
	req := eventType.api.EventTypeAPI.V1EventTypeDelete(
		ctx,
		eventTypeName,
	)

	if options != nil {
		if options.Expunge != nil {
			req = req.Expunge(*options.Expunge)
		}
	}

	res, err := req.Execute()
	return wrapError(err, res)
}

// Partially update an event type.
func (eventType *EventType) Patch(
	ctx context.Context,
	eventTypeName string,
	eventTypePatch *EventTypePatch,
) (*EventTypeOut, error) {
	req := eventType.api.EventTypeAPI.V1EventTypePatch(
		ctx,
		eventTypeName,
	).EventTypePatch(*eventTypePatch)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}
