package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type Message struct {
	api *openapi.APIClient
}

type MessageListOptions struct {
	// Limit the number of returned items
	Limit *int32
	// The iterator returned from a prior invocation
	Iterator *string
	// Filter response based on the channel.
	Channel *string
	// Only include items created before a certain date.
	Before *time.Time
	// Only include items created after a certain date.
	After *time.Time
	// When `true` message payloads are included in the response.
	WithContent *bool
	// Filter messages matching the provided tag.
	Tag *string
	// Filter response based on the event type
	EventTypes *[]string
}

// List all of the application's messages.
//
// The `before` and `after` parameters let you filter all items created before or after a certain date. These can be used alongside an iterator to paginate over results
// within a certain window.
//
// Note that by default this endpoint is limited to retrieving 90 days' worth of data
// relative to now or, if an iterator is provided, 90 days before/after the time indicated
// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
// set the `before` or `after` parameter as appropriate.
func (message *Message) List(
	ctx context.Context,
	appId string,
	options *MessageListOptions,
) (*ListResponseMessageOut, error) {
	req := message.api.MessageAPI.V1MessageList(
		ctx,
		appId,
	)

	if options != nil {
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Channel != nil {
			req = req.Channel(*options.Channel)
		}
		if options.Before != nil {
			req = req.Before(*options.Before)
		}
		if options.After != nil {
			req = req.After(*options.After)
		}
		if options.WithContent != nil {
			req = req.WithContent(*options.WithContent)
		}
		if options.Tag != nil {
			req = req.Tag(*options.Tag)
		}
		if options.EventTypes != nil {
			req = req.EventTypes(*options.EventTypes)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Creates a new message and dispatches it to all of the application's endpoints.
//
// The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made.
// If a message with the same `eventId` already exists for the application, a 409 conflict error will be returned.
//
// The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types.
// Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.
//
// The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
func (message *Message) Create(
	ctx context.Context,
	appId string,
	messageIn *MessageIn,
) (*MessageOut, error) {
	return message.CreateWithOptions(
		ctx,
		appId,
		messageIn,
		nil,
	)
}

// Creates a new message and dispatches it to all of the application's endpoints.
//
// The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made.
// If a message with the same `eventId` already exists for the application, a 409 conflict error will be returned.
//
// The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types.
// Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.
//
// The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
func (message *Message) CreateWithOptions(
	ctx context.Context,
	appId string,
	messageIn *MessageIn,
	options *PostOptions,
) (*MessageOut, error) {
	req := message.api.MessageAPI.V1MessageCreate(
		ctx,
		appId,
	).MessageIn(*messageIn)

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

// Get a message by its ID or eventID.
func (message *Message) Get(
	ctx context.Context,
	appId string,
	msgId string,
) (*MessageOut, error) {
	req := message.api.MessageAPI.V1MessageGet(
		ctx,
		appId,
		msgId,
	)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Delete the given message's payload.
//
// Useful in cases when a message was accidentally sent with sensitive content.
// The message can't be replayed or resent once its payload has been deleted or expired.
func (message *Message) ExpungeContent(
	ctx context.Context,
	appId string,
	msgId string,
) error {
	req := message.api.MessageAPI.V1MessageExpungeContent(
		ctx,
		appId,
		msgId,
	)

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
func NewMessageInRaw(
	eventType string,
	payload string,
	contentType openapi.NullableString,
) *MessageIn {
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
