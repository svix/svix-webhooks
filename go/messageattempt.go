package svix

import (
	"context"

	"github.com/svixhq/svix-libs/go/internal/openapi"
)

type MessageAttempt struct {
	api *openapi.APIClient
}

type FetchOptionsMessageAttempt struct{}

type (
	ListResponseMessageAttemptOut         openapi.ListResponseMessageAttemptOut
	MessageAttemptOut                     openapi.MessageAttemptOut
	ListResponseEndpointMessageOut        openapi.ListResponseEndpointMessageOut
	ListResponseMessageEndpointOut        openapi.ListResponseMessageEndpointOut
	ListResponseMessageAttemptEndpointOut openapi.ListResponseMessageAttemptEndpointOut
)

func (m *MessageAttempt) List(appID string, msgID string, options FetchOptionsMessageAttempt) (*ListResponseMessageAttemptOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(context.TODO(), msgID, appID)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseMessageAttemptOut(out)
	return &ret, nil
}

func (m *MessageAttempt) Get(appID string, msgID string, attemptID string) (*MessageAttemptOut, error) {
	req := m.api.MessageAttemptApi.GetAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(context.TODO(), attemptID, msgID, appID)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := MessageAttemptOut(out)
	return &ret, nil
}

func (m *MessageAttempt) Resend(appID string, msgID string, endpointID string) (map[string]interface{}, error) {
	req := m.api.MessageAttemptApi.ResendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(context.TODO(), endpointID, msgID, appID)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	// TODO Why is this not a MessageAttemptOut?
	return out, nil
}

func (m *MessageAttempt) ListAttemptedMessages(appID string, endpointID string, options FetchOptionsMessageAttempt) (*ListResponseEndpointMessageOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(context.TODO(), endpointID, appID)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseEndpointMessageOut(out)
	return &ret, nil
}

func (m *MessageAttempt) ListAttemptedDestinations(appID string, msgID string, options FetchOptionsMessageAttempt) (*ListResponseMessageEndpointOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(context.TODO(), msgID, appID)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseMessageEndpointOut(out)
	return &ret, nil
}

func (m *MessageAttempt) ListAttemptsForEndpoint(appID string, msgID string, endpointID string, options FetchOptionsMessageAttempt) (*ListResponseMessageAttemptEndpointOut, error) {
	req := m.api.MessageAttemptApi.ListAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(context.TODO(), msgID, endpointID, appID)
	out, _, err := req.Execute()
	if err != nil {
		return nil, err
	}
	ret := ListResponseMessageAttemptEndpointOut(out)
	return &ret, nil
}
