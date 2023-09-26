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
	Iterator        *string
	Limit           *int32
	WithContent     *bool
	IncludeArchived *bool
}

func (e *EventType) List(ctx context.Context, options *EventTypeListOptions) (*ListResponseEventTypeOut, error) {
	req := e.api.EventTypeApi.V1EventTypeList(ctx)
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
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseEventTypeOut(out)
	return &ret, nil
}

func (e *EventType) Create(ctx context.Context, eventTypeIn *EventTypeIn) (*EventTypeOut, error) {
	return e.CreateWithOptions(ctx, eventTypeIn, nil)
}

func (e *EventType) CreateWithOptions(ctx context.Context, eventTypeIn *EventTypeIn, options *PostOptions) (*EventTypeOut, error) {
	req := e.api.EventTypeApi.V1EventTypeCreate(ctx)
	req = req.EventTypeIn(openapi.EventTypeIn(*eventTypeIn))
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EventTypeOut(out)
	return &ret, nil
}

func (e *EventType) Get(ctx context.Context, eventTypeName string) (*EventTypeOut, error) {
	req := e.api.EventTypeApi.V1EventTypeGet(ctx, eventTypeName)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EventTypeOut(out)
	return &ret, nil
}

func (e *EventType) Update(ctx context.Context, eventTypeName string, eventTypeUpdate *EventTypeUpdate) (*EventTypeOut, error) {
	req := e.api.EventTypeApi.V1EventTypeUpdate(ctx, eventTypeName)
	req = req.EventTypeUpdate(openapi.EventTypeUpdate(*eventTypeUpdate))
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EventTypeOut(out)
	return &ret, nil
}

func (e *EventType) Patch(ctx context.Context, eventTypeName string, eventTypePatch *EventTypePatch) (*EventTypeOut, error) {
	req := e.api.EventTypeApi.V1EventTypePatch(ctx, eventTypeName)
	req = req.EventTypePatch(openapi.EventTypePatch(*eventTypePatch))
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EventTypeOut(out)
	return &ret, nil
}

func (e *EventType) Delete(ctx context.Context, eventTypeName string) error {
	return e.DeleteWithOptions(ctx, eventTypeName, nil)
}

type EventTypeDeleteOptions struct {
	Expunge *bool
}

func (e *EventType) DeleteWithOptions(ctx context.Context, eventTypeName string, options *EventTypeDeleteOptions) error {
	req := e.api.EventTypeApi.V1EventTypeDelete(ctx, eventTypeName)
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
	req := e.api.EventTypeApi.V1EventTypeImportOpenapi(ctx).EventTypeImportOpenApiIn(eventTypeImportOpenApiIn)
	if options != nil && options.IdempotencyKey != nil {
		req = req.IdempotencyKey(*options.IdempotencyKey)
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EventTypeImportOpenApiOut(out)
	return &ret, nil
}
