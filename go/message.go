package svix

import (
	"context"
	"time"

	"github.com/svix/svix-libs/go/internal/openapi"
)

type Message struct {
	api *openapi.APIClient
}

type (
	ListResponseMessageOut openapi.ListResponseMessageOut
	MessageIn              openapi.MessageIn
	MessageOut             openapi.MessageOut
)

type MessageListOptions struct {
	Iterator   *string
	Limit      *int32
	EventTypes *[]string
	Before     *time.Time
}

func (m *Message) List(appId string, options *MessageListOptions) (*ListResponseMessageOut, error) {
	req := m.api.MessageApi.ListMessagesApiV1AppAppIdMsgGet(context.Background(), appId)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.EventTypes != nil {
			req = req.EventTypes(*options.EventTypes)
		}
		if options.Before != nil {
			req = req.Before(*options.Before)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseMessageOut(out)
	return &ret, nil
}

func (m *Message) Create(appId string, messageIn *MessageIn) (*MessageOut, error) {
	req := m.api.MessageApi.CreateMessageApiV1AppAppIdMsgPost(context.Background(), appId)
	req = req.MessageIn(openapi.MessageIn(*messageIn))
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := MessageOut(out)
	return &ret, nil
}

func (m *Message) Get(appId string, msgId string) (*MessageOut, error) {
	req := m.api.MessageApi.GetMessageApiV1AppAppIdMsgMsgIdGet(context.Background(), msgId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := MessageOut(out)
	return &ret, nil
}
