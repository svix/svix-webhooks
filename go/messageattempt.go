package svix

import (
	"context"

	"github.com/svix/svix-libs/go/internal/openapi"
)

type MessageAttempt struct {
	api *openapi.APIClient
}

type MessageStatus openapi.MessageStatus

type FetchOptionsMessageAttempt struct {
	FetchOptions
	Status *MessageStatus
}

type (
	ListResponseMessageAttemptOut         openapi.ListResponseMessageAttemptOut
	MessageAttemptOut                     openapi.MessageAttemptOut
	ListResponseEndpointMessageOut        openapi.ListResponseEndpointMessageOut
	ListResponseMessageEndpointOut        openapi.ListResponseMessageEndpointOut
	ListResponseMessageAttemptEndpointOut openapi.ListResponseMessageAttemptEndpointOut
)

func (m *MessageAttempt) List(appId string, msgId string, options *FetchOptionsMessageAttempt) (*ListResponseMessageAttemptOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(context.Background(), msgId, appId)
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
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseMessageAttemptOut(out)
	return &ret, nil
}

func (m *MessageAttempt) Get(appId string, msgId string, attemptID string) (*MessageAttemptOut, error) {
	req := m.api.MessageAttemptApi.GetAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(context.Background(), attemptID, msgId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := MessageAttemptOut(out)
	return &ret, nil
}

func (m *MessageAttempt) Resend(appId string, msgId string, endpointId string) error {
	req := m.api.MessageAttemptApi.ResendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(context.Background(), endpointId, msgId, appId)
	res, err := req.Execute()
	return wrapError(err, res)
}

func (m *MessageAttempt) ListAttemptedMessages(appId string, endpointId string, options *FetchOptionsMessageAttempt) (*ListResponseEndpointMessageOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(context.Background(), endpointId, appId)
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
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseEndpointMessageOut(out)
	return &ret, nil
}

func (m *MessageAttempt) ListAttemptedDestinations(appId string, msgId string, options *FetchOptions) (*ListResponseMessageEndpointOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(context.Background(), msgId, appId)
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

func (m *MessageAttempt) ListAttemptsForEndpoint(appId string, msgId string, endpointId string, options FetchOptionsMessageAttempt) (*ListResponseMessageAttemptEndpointOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(context.Background(), msgId, appId, endpointId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseMessageAttemptEndpointOut(out)
	return &ret, nil
}
