package svix

import (
	"context"

	"github.com/svixhq/svix-libs/go/internal/openapi"
)

type EventType struct {
	api *openapi.APIClient
}

type (
	ListResponseEventTypeInOut openapi.ListResponseEventTypeInOut
	EventTypeInOut             openapi.EventTypeInOut
	EventTypeUpdate            openapi.EventTypeUpdate
)

func (e *EventType) list(options FetchOptions) (*ListResponseEventTypeInOut, error) {
	req := e.api.EventTypeApi.ListEventTypesApiV1EventTypeGet(context.Background())
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseEventTypeInOut(out)
	return &ret, nil
}

func (e *EventType) Create(eventTypeInOut *EventTypeInOut) (*EventTypeInOut, error) {
	req := e.api.EventTypeApi.CreateEventTypeApiV1EventTypePost(context.Background())
	req = req.EventTypeInOut(openapi.EventTypeInOut(*eventTypeInOut))
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := EventTypeInOut(out)
	return &ret, nil
}

func (e *EventType) Update(eventTypeName string, eventTypeUpdate *EventTypeUpdate) (*EventTypeInOut, error) {
	req := e.api.EventTypeApi.UpdateEventTypeApiV1EventTypeEventTypeNamePut(context.Background(), eventTypeName)
	req = req.EventTypeUpdate(openapi.EventTypeUpdate(*eventTypeUpdate))
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := EventTypeInOut(out)
	return &ret, nil
}

func (e *EventType) Delete(eventTypeName string) error {
	req := e.api.EventTypeApi.DeleteEventTypeApiV1EventTypeEventTypeNameDelete(context.Background(), eventTypeName)
	_, err := req.Execute()
	return err
}
