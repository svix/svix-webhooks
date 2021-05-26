package svix

import (
	"context"

	"github.com/svixhq/svix-libs/go/internal/openapi"
)

type Message struct {
	api *openapi.APIClient
}

type (
	ListResponseMessageOut openapi.ListResponseMessageOut
	MessageIn              openapi.MessageIn
	MessageOut             openapi.MessageOut
)

func (m *Message) List(appId string, options *FetchOptions) (*ListResponseMessageOut, error) {
	req := m.api.MessageApi.ListMessagesApiV1AppAppIdMsgGet(context.Background(), appId)
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
	ret := ListResponseMessageOut(out)
	return &ret, nil
}

func (m *Message) Create(appId string, messageIn *MessageIn) (*MessageOut, error) {
	req := m.api.MessageApi.CreateMessageApiV1AppAppIdMsgPost(context.Background(), appId)
	req = req.MessageIn(openapi.MessageIn(*messageIn))
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := MessageOut(out)
	return &ret, nil
}

func (m *Message) Get(appId string, msgId string) (*MessageOut, error) {
	req := m.api.MessageApi.GetMessageApiV1AppAppIdMsgMsgIdGet(context.Background(), msgId, appId)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := MessageOut(out)
	return &ret, nil
}
