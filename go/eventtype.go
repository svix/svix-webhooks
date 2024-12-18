package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type EventType struct {
	api *openapi.APIClient
}

type (
	ListResponseEventTypeOut      = openapi.ListResponseEventTypeOut
	EventTypeIn                   = openapi.EventTypeIn
	EventTypeOut                  = openapi.EventTypeOut
	EventTypePatch                = openapi.EventTypePatch
	EventTypeUpdate               = openapi.EventTypeUpdate
	EventTypeImportOpenApiIn      = openapi.EventTypeImportOpenApiIn
	EventTypeImportOpenApiOut     = openapi.EventTypeImportOpenApiOut
	EventTypeImportOpenApiOutData = openapi.EventTypeImportOpenApiOutData
)

type EventTypeListOptions struct {
	// Limit the number of returned items
	Limit *int32
	// The iterator returned from a prior invocation
	Iterator *string
	// The sorting order of the returned items
	Order *Ordering
	// When `true` archived (deleted but not expunged) items are included in the response
	IncludeArchived *bool
	// When `true` the full item (including the schema) is included in the response
	WithContent *bool
}

func (e *EventType) List(ctx context.Context, options *EventTypeListOptions) (*ListResponseEventTypeOut, error) {
	req := e.api.EventTypeAPI.V1EventTypeList(ctx)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.WithContent != nil {
			req = req.WithContent(*options.WithContent)
		}
		if options.IncludeArchived != nil {
			req = req.IncludeArchived(*options.IncludeArchived)
		}
		if options.Order != nil {
			req = req.Order(*options.Order)
		}
	}
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *EventType) Create(ctx context.Context, eventTypeIn *EventTypeIn) (*EventTypeOut, error) {
	return e.CreateWithOptions(ctx, eventTypeIn, nil)
}

func (e *EventType) CreateWithOptions(ctx context.Context, eventTypeIn *EventTypeIn, options *PostOptions) (*EventTypeOut, error) {
	req := e.api.EventTypeAPI.V1EventTypeCreate(ctx)
	req = req.EventTypeIn(*eventTypeIn)
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

func (e *EventType) Get(ctx context.Context, eventTypeName string) (*EventTypeOut, error) {
	req := e.api.EventTypeAPI.V1EventTypeGet(ctx, eventTypeName)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *EventType) Update(ctx context.Context, eventTypeName string, eventTypeUpdate *EventTypeUpdate) (*EventTypeOut, error) {
	req := e.api.EventTypeAPI.V1EventTypeUpdate(ctx, eventTypeName)
	req = req.EventTypeUpdate(*eventTypeUpdate)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *EventType) Patch(ctx context.Context, eventTypeName string, eventTypePatch *EventTypePatch) (*EventTypeOut, error) {
	req := e.api.EventTypeAPI.V1EventTypePatch(ctx, eventTypeName)
	req = req.EventTypePatch(*eventTypePatch)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *EventType) Delete(ctx context.Context, eventTypeName string) error {
	return e.DeleteWithOptions(ctx, eventTypeName, nil)
}

type EventTypeDeleteOptions struct {
	Expunge *bool
}

func (e *EventType) DeleteWithOptions(ctx context.Context, eventTypeName string, options *EventTypeDeleteOptions) error {
	req := e.api.EventTypeAPI.V1EventTypeDelete(ctx, eventTypeName)
	if options != nil {
		if options.Expunge != nil {
			req = req.Expunge(*options.Expunge)
		}
	}
	res, err := req.Execute()
	return wrapError(err, res)
}

func (e *EventType) ImportOpenApi(ctx context.Context, eventTypeImportOpenApiIn EventTypeImportOpenApiIn) (*EventTypeImportOpenApiOut, error) {
	return e.ImportOpenApiWithOptions(ctx, eventTypeImportOpenApiIn, nil)
}

func (e *EventType) ImportOpenApiWithOptions(ctx context.Context, eventTypeImportOpenApiIn EventTypeImportOpenApiIn, options *PostOptions) (*EventTypeImportOpenApiOut, error) {
	req := e.api.EventTypeAPI.V1EventTypeImportOpenapi(ctx).EventTypeImportOpenApiIn(eventTypeImportOpenApiIn)
	if options != nil && options.IdempotencyKey != nil {
		req = req.IdempotencyKey(*options.IdempotencyKey)
	}
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}
