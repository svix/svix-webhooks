package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type MessageAttempt struct {
	api *openapi.APIClient
}

type MessageStatus openapi.MessageStatus
type StatusCodeClass openapi.StatusCodeClass

type (
	ListResponseMessageAttemptOut         openapi.ListResponseMessageAttemptOut
	MessageAttemptOut                     openapi.MessageAttemptOut
	ListResponseEndpointMessageOut        openapi.ListResponseEndpointMessageOut
	ListResponseMessageEndpointOut        openapi.ListResponseMessageEndpointOut
	ListResponseMessageAttemptEndpointOut openapi.ListResponseMessageAttemptEndpointOut
)

type MessageAttemptListOptions struct {
	Iterator        *string
	Limit           *int32
	Status          *MessageStatus
	EventTypes      *[]string
	Before          *time.Time
	After           *time.Time
	StatusCodeClass *StatusCodeClass
	Channel         *string
}

// Deprecated: use `ListByMsg` or `ListByEndpoint` instead
func (m *MessageAttempt) List(ctx context.Context, appId string, msgId string, options *MessageAttemptListOptions) (*ListResponseMessageAttemptOut, error) {
	return m.ListByMsg(ctx, appId, msgId, options)
}

func (m *MessageAttempt) ListByMsg(ctx context.Context, appId string, msgId string, options *MessageAttemptListOptions) (*ListResponseMessageAttemptOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptsByMsgApiV1AppAppIdAttemptMsgMsgIdGet(ctx, appId, msgId)
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
		if options.StatusCodeClass != nil {
			req.StatusCodeClass(openapi.StatusCodeClass(*options.StatusCodeClass))
		}
		if options.Channel != nil {
			req = req.Channel(*options.Channel)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseMessageAttemptOut(out)
	return &ret, nil
}

func (m *MessageAttempt) ListByEndpoint(ctx context.Context, appId string, endpointId string, options *MessageAttemptListOptions) (*ListResponseMessageAttemptOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptsByEndpointApiV1AppAppIdAttemptEndpointEndpointIdGet(ctx, appId, endpointId)
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
		if options.StatusCodeClass != nil {
			req.StatusCodeClass(openapi.StatusCodeClass(*options.StatusCodeClass))
		}
		if options.Channel != nil {
			req = req.Channel(*options.Channel)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseMessageAttemptOut(out)
	return &ret, nil
}

func (m *MessageAttempt) Get(ctx context.Context, appId string, msgId string, attemptID string) (*MessageAttemptOut, error) {
	req := m.api.MessageAttemptApi.GetAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(ctx, attemptID, msgId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := MessageAttemptOut(out)
	return &ret, nil
}

func (m *MessageAttempt) Resend(ctx context.Context, appId string, msgId string, endpointId string) error {
	return m.ResendWithOptions(ctx, appId, msgId, endpointId, nil)
}

func (m *MessageAttempt) ResendWithOptions(ctx context.Context, appId string, msgId string, endpointId string, options *PostOptions) error {
	req := m.api.MessageAttemptApi.ResendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(ctx, endpointId, msgId, appId)
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	res, err := req.Execute()
	return wrapError(err, res)
}

func (m *MessageAttempt) ListAttemptedMessages(ctx context.Context, appId string, endpointId string, options *MessageAttemptListOptions) (*ListResponseEndpointMessageOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(ctx, endpointId, appId)
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
	ret := ListResponseEndpointMessageOut(out)
	return &ret, nil
}

func (m *MessageAttempt) ListAttemptedDestinations(ctx context.Context, appId string, msgId string, options *MessageAttemptListOptions) (*ListResponseMessageEndpointOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(ctx, msgId, appId)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseMessageEndpointOut(out)
	return &ret, nil
}

func (m *MessageAttempt) ListAttemptsForEndpoint(ctx context.Context, appId string, msgId string, endpointId string, options *MessageAttemptListOptions) (*ListResponseMessageAttemptEndpointOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(ctx, msgId, appId, endpointId)
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
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseMessageAttemptEndpointOut(out)
	return &ret, nil
}

func (m *MessageAttempt) ExpungeContent(ctx context.Context, appId string, msgId string, attemptId string) error {
	req := m.api.MessageAttemptApi.ExpungeAttemptContentApiV1AppAppIdMsgMsgIdAttemptAttemptIdContentDelete(ctx, attemptId, msgId, appId)
	res, err := req.Execute()
	return wrapError(err, res)
}
