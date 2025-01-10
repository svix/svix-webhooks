package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type MessageAttempt struct {
	api *openapi.APIClient
}

type MessageAttemptListOptions struct {
	Iterator        *string
	Limit           *int32
	Status          *MessageStatus
	EventTypes      *[]string
	Before          *time.Time
	After           *time.Time
	StatusCodeClass *StatusCodeClass
	Channel         *string
	EndpointId      *string
	WithContent     *bool
	WithMsg         *bool
	Tag             *string
}

// Deprecated: use `ListByMsg` or `ListByEndpoint` instead
func (messageAttempt *MessageAttempt) List(
	ctx context.Context,
	appId string,
	msgId string,
	options *MessageAttemptListOptions,
) (*ListResponseMessageAttemptOut, error) {
	return messageAttempt.ListByMsg(ctx, appId, msgId, options)
}

// List attempts by endpoint id
//
// Note that by default this endpoint is limited to retrieving 90 days' worth of data
// relative to now or, if an iterator is provided, 90 days before/after the time indicated
// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
// set the `before` or `after` parameter as appropriate.
func (messageAttempt *MessageAttempt) ListByEndpoint(
	ctx context.Context,
	appId string,
	endpointId string,
	options *MessageAttemptListOptions,
) (*ListResponseMessageAttemptOut, error) {
	req := messageAttempt.api.MessageAttemptAPI.V1MessageAttemptListByEndpoint(
		ctx,
		appId,
		endpointId,
	)

	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Status != nil {
			req = req.Status(*options.Status)
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
		if options.StatusCodeClass != nil {
			req.StatusCodeClass(*options.StatusCodeClass)
		}
		if options.Channel != nil {
			req = req.Channel(*options.Channel)
		}
		if options.WithContent != nil {
			req = req.WithContent(*options.WithContent)
		}
		if options.WithMsg != nil {
			req = req.WithMsg(*options.WithMsg)
		}
		if options.Tag != nil {
			req = req.Tag(*options.Tag)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// List attempts by message ID.
//
// Note that by default this endpoint is limited to retrieving 90 days' worth of data
// relative to now or, if an iterator is provided, 90 days before/after the time indicated
// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
// set the `before` or `after` parameter as appropriate.
func (messageAttempt *MessageAttempt) ListByMsg(
	ctx context.Context,
	appId string,
	msgId string,
	options *MessageAttemptListOptions,
) (*ListResponseMessageAttemptOut, error) {
	req := messageAttempt.api.MessageAttemptAPI.V1MessageAttemptListByMsg(ctx, appId, msgId)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Status != nil {
			req = req.Status(*options.Status)
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
		if options.StatusCodeClass != nil {
			req.StatusCodeClass(*options.StatusCodeClass)
		}
		if options.Channel != nil {
			req = req.Channel(*options.Channel)
		}
		if options.EndpointId != nil {
			req = req.EndpointId(*options.EndpointId)
		}
		if options.WithContent != nil {
			req = req.WithContent(*options.WithContent)
		}
		if options.Tag != nil {
			req = req.Tag(*options.Tag)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.
//
// The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
//
// Note that by default this endpoint is limited to retrieving 90 days' worth of data
// relative to now or, if an iterator is provided, 90 days before/after the time indicated
// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
// set the `before` or `after` parameter as appropriate.
func (messageAttempt *MessageAttempt) ListAttemptedMessages(
	ctx context.Context,
	appId string,
	endpointId string,
	options *MessageAttemptListOptions,
) (*ListResponseEndpointMessageOut, error) {
	req := messageAttempt.api.MessageAttemptAPI.V1MessageAttemptListAttemptedMessages(
		ctx,
		appId,
		endpointId,
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
		if options.Tag != nil {
			req = req.Tag(*options.Tag)
		}
		if options.Status != nil {
			req = req.Status(*options.Status)
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

// `msg_id`: Use a message id or a message `eventId`
func (messageAttempt *MessageAttempt) Get(
	ctx context.Context,
	appId string,
	msgId string,
	attemptId string,
) (*MessageAttemptOut, error) {
	req := messageAttempt.api.MessageAttemptAPI.V1MessageAttemptGet(
		ctx,
		appId,
		msgId,
		attemptId,
	)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Deletes the given attempt's response body.
//
// Useful when an endpoint accidentally returned sensitive content.
// The message can't be replayed or resent once its payload has been deleted or expired.
func (messageAttempt *MessageAttempt) ExpungeContent(
	ctx context.Context,
	appId string,
	msgId string,
	attemptId string,
) error {
	req := messageAttempt.api.MessageAttemptAPI.V1MessageAttemptExpungeContent(
		ctx,
		appId,
		msgId,
		attemptId,
	)

	res, err := req.Execute()
	return wrapError(err, res)
}

// List endpoints attempted by a given message.
//
// Additionally includes metadata about the latest message attempt.
// By default, endpoints are listed in ascending order by ID.
func (messageAttempt *MessageAttempt) ListAttemptedDestinations(
	ctx context.Context,
	appId string,
	msgId string,
	options *MessageAttemptListOptions,
) (*ListResponseMessageEndpointOut, error) {
	req := messageAttempt.api.MessageAttemptAPI.V1MessageAttemptListAttemptedDestinations(ctx, appId, msgId)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
	}
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

// Deprecated: use `ListByMsg` instead, passing the endpoint ID through options
func (messageAttempt *MessageAttempt) ListAttemptsForEndpoint(
	ctx context.Context,
	appId string,
	msgId string,
	endpointId string,
	options *MessageAttemptListOptions,
) (*ListResponseMessageAttemptEndpointOut, error) {
	req := messageAttempt.api.MessageAttemptAPI.V1MessageAttemptListByEndpointDeprecated(ctx, appId, msgId, endpointId)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Status != nil {
			req = req.Status(openapi.MessageStatus(*options.Status))
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
	}
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Resend a message to the specified endpoint.
func (messageAttempt *MessageAttempt) Resend(
	ctx context.Context,
	appId string,
	msgId string,
	endpointId string,
) error {
	return messageAttempt.ResendWithOptions(
		ctx,
		appId,
		msgId,
		endpointId,
		nil,
	)
}

// Resend a message to the specified endpoint.
func (messageAttempt *MessageAttempt) ResendWithOptions(
	ctx context.Context,
	appId string,
	msgId string,
	endpointId string,
	options *PostOptions,
) error {
	req := messageAttempt.api.MessageAttemptAPI.V1MessageAttemptResend(
		ctx,
		appId,
		msgId,
		endpointId,
	)

	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}

	res, err := req.Execute()
	return wrapError(err, res)
}
