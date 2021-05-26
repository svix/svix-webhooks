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

func (m *Message) List(appID string, options FetchOptions) (*ListResponseMessageOut, error) {
	req := m.api.MessageApi.ListMessagesApiV1AppAppIdMsgGet(context.Background(), appID)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseMessageOut(out)
	return &ret, nil
}

func (m *Message) Create(appID string, messageIn MessageIn) (*MessageOut, error) {
	req := m.api.MessageApi.CreateMessageApiV1AppAppIdMsgPost(context.Background(), appID)
	req = req.MessageIn(openapi.MessageIn(messageIn))
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := MessageOut(out)
	return &ret, nil
}

func (m *Message) Get(appID string, msgID string) (*MessageOut, error) {
	req := m.api.MessageApi.GetMessageApiV1AppAppIdMsgMsgIdGet(context.Background(), msgID, appID)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := MessageOut(out)
	return &ret, nil
}
