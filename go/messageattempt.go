package svix

import (
	"context"

	"github.com/svixhq/svix-libs/go/internal/openapi"
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

func (m *MessageAttempt) List(appID string, msgID string, options *FetchOptionsMessageAttempt) (*ListResponseMessageAttemptOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(context.Background(), msgID, appID)
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
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseMessageAttemptOut(out)
	return &ret, nil
}

func (m *MessageAttempt) Get(appID string, msgID string, attemptID string) (*MessageAttemptOut, error) {
	req := m.api.MessageAttemptApi.GetAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(context.Background(), attemptID, msgID, appID)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := MessageAttemptOut(out)
	return &ret, nil
}

func (m *MessageAttempt) Resend(appID string, msgID string, endpointID string) error {
	req := m.api.MessageAttemptApi.ResendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(context.Background(), endpointID, msgID, appID)
	_, _, err := req.Execute()
	return err
}

func (m *MessageAttempt) ListAttemptedMessages(appID string, endpointID string, options *FetchOptionsMessageAttempt) (*ListResponseEndpointMessageOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(context.Background(), endpointID, appID)
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
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseEndpointMessageOut(out)
	return &ret, nil
}

func (m *MessageAttempt) ListAttemptedDestinations(appID string, msgID string, options *FetchOptions) (*ListResponseMessageEndpointOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(context.Background(), msgID, appID)
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
	ret := ListResponseMessageEndpointOut(out)
	return &ret, nil
}

func (m *MessageAttempt) ListAttemptsForEndpoint(appID string, msgID string, endpointID string, options FetchOptionsMessageAttempt) (*ListResponseMessageAttemptEndpointOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(context.Background(), msgID, endpointID, appID)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseMessageAttemptEndpointOut(out)
	return &ret, nil
}
