package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/internal/openapi"
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
	After      *time.Time
	Channel    *string
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
		if options.After != nil {
			req = req.After(*options.After)
		}
		if options.Channel != nil {
			req = req.Channel(*options.Channel)
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
	return m.CreateWithOptions(appId, messageIn, nil)
}

func (m *Message) CreateWithOptions(appId string, messageIn *MessageIn, options *PostOptions) (*MessageOut, error) {
	req := m.api.MessageApi.CreateMessageApiV1AppAppIdMsgPost(context.Background(), appId)
	req = req.MessageIn(openapi.MessageIn(*messageIn))
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
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
