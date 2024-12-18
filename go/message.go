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
	ListResponseMessageOut = openapi.ListResponseMessageOut
	MessageIn              = openapi.MessageIn
	MessageOut             = openapi.MessageOut
)

type MessageListOptions struct {
	// Limit the number of returned items
	Limit *int32
	// The iterator returned from a prior invocation
	Iterator *string
	// Filter response based on the channel
	Channel *string
	// Only include items created before a certain date
	Before *time.Time
	// Only include items created after a certain date
	After *time.Time
	// When `true` message payloads are included in the response
	WithContent *bool
	// Filter messages matching the provided tag
	Tag *string
	// Filter response based on the event type
	EventTypes *[]string
}

func (m *Message) List(ctx context.Context, appId string, options *MessageListOptions) (*ListResponseMessageOut, error) {
	req := m.api.MessageAPI.V1MessageList(ctx, appId)
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
		if options.Tag != nil {
			req = req.Tag(*options.Tag)
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

func (m *Message) Create(ctx context.Context, appId string, messageIn *MessageIn) (*MessageOut, error) {
	return m.CreateWithOptions(ctx, appId, messageIn, nil)
}

func (m *Message) CreateWithOptions(ctx context.Context, appId string, messageIn *MessageIn, options *PostOptions) (*MessageOut, error) {
	req := m.api.MessageAPI.V1MessageCreate(ctx, appId)
	req = req.MessageIn(*messageIn)
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

func (m *Message) Get(ctx context.Context, appId string, msgId string) (*MessageOut, error) {
	req := m.api.MessageAPI.V1MessageGet(ctx, appId, msgId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (m *Message) ExpungeContent(ctx context.Context, appId string, msgId string) error {
	req := m.api.MessageAPI.V1MessageExpungeContent(ctx, appId, msgId)
	res, err := req.Execute()
	return wrapError(err, res)
}

// Instantiates a new MessageIn object with a raw string payload.
//
// The payload is not normalized on the server. Normally, payloads are required
// to be JSON, and Svix will minify the payload before sending the webhook
// (for example, by removing extraneous whitespace or unnecessarily escaped
// characters in strings). With this function, the payload will be sent
// "as is", without any minification or other processing.
//
// The `contentType` parameter can be used to change the `content-type` header
// of the webhook sent by Svix overriding the default of `application/json`.
//
// See the class documentation for details about the other parameters.
func NewMessageInRaw(eventType string, payload string, contentType openapi.NullableString) *MessageIn {
	msgIn := openapi.NewMessageIn(eventType, make(map[string]interface{}))

	transformationsParams := map[string]interface{}{
		"rawPayload": payload,
	}
	if contentType.IsSet() {
		transformationsParams["headers"] = map[string]string{
			"content-type": *contentType.Get(),
		}
	}
	msgIn.SetTransformationsParams(transformationsParams)

	return msgIn
}
