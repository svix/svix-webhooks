package svix

import (
	"context"

	"github.com/svix/svix-libs/go/internal/openapi"
)

type EventType struct {
	api *openapi.APIClient
}

type (
	ListResponseEventTypeOut openapi.ListResponseEventTypeOut
	EventTypeIn              openapi.EventTypeIn
	EventTypeOut             openapi.EventTypeOut
	EventTypeUpdate          openapi.EventTypeUpdate
)

func (e *EventType) List(options *FetchOptions) (*ListResponseEventTypeOut, error) {
	req := e.api.EventTypeApi.ListEventTypesApiV1EventTypeGet(context.Background())
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
	}
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseEventTypeOut(out)
	return &ret, nil
}

func (e *EventType) Create(eventTypeIn *EventTypeIn) (*EventTypeOut, error) {
	req := e.api.EventTypeApi.CreateEventTypeApiV1EventTypePost(context.Background())
	req = req.EventTypeIn(openapi.EventTypeIn(*eventTypeIn))
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := EventTypeOut(out)
	return &ret, nil
}

func (e *EventType) Update(eventTypeName string, eventTypeUpdate *EventTypeUpdate) (*EventTypeOut, error) {
	req := e.api.EventTypeApi.UpdateEventTypeApiV1EventTypeEventTypeNamePut(context.Background(), eventTypeName)
	req = req.EventTypeUpdate(openapi.EventTypeUpdate(*eventTypeUpdate))
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := EventTypeOut(out)
	return &ret, nil
}

func (e *EventType) Delete(eventTypeName string) error {
	req := e.api.EventTypeApi.DeleteEventTypeApiV1EventTypeEventTypeNameDelete(context.Background(), eventTypeName)
	_, err := req.Execute()
	return err
}
