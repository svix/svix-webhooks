// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/v2/go/internal"
	"github.com/svix/svix-webhooks/v2/go/models"
)

type EndpointMcpEvents struct {
	client *internal.SvixHttpClient
}

func newEndpointMcpEvents(client *internal.SvixHttpClient) EndpointMcpEvents {
	return EndpointMcpEvents{client}
}

type EndpointMcpEventsSubscribeOptions struct {
	IdempotencyKey *string
}

type EndpointMcpEventsUnsubscribeOptions struct {
	IdempotencyKey *string
}

// Create or refresh an MCP Events subscription.
func (endpointMcpEvents EndpointMcpEvents) Subscribe(
	ctx context.Context,
	appId string,
	mcpEventsSubscribeIn models.McpEventsSubscribeIn,
	o *EndpointMcpEventsSubscribeOptions,
) (*models.McpEventsSubscribeOut, error) {
	var err error
	pathMap := map[string]string{
		"app_id": appId,
	}
	headerMap := map[string]string{}
	if o == nil {
		opts := EndpointMcpEventsSubscribeOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return nil, err
	}
	return internal.ExecuteRequest[models.McpEventsSubscribeIn, models.McpEventsSubscribeOut](
		ctx,
		endpointMcpEvents.client,
		"POST",
		"/api/v1/app/{app_id}/mcp-events/subscribe",
		pathMap,
		nil,
		headerMap,
		&mcpEventsSubscribeIn,
	)
}

// Stop an MCP Events subscription.
//
// Forward an MCP server's `events/unsubscribe` requests here. The
// subscription's endpoint is deleted. Unsubscribing from a subscription that
// doesn't exist succeeds, so this can be repeated.
func (endpointMcpEvents EndpointMcpEvents) Unsubscribe(
	ctx context.Context,
	appId string,
	mcpEventsUnsubscribeIn models.McpEventsUnsubscribeIn,
	o *EndpointMcpEventsUnsubscribeOptions,
) error {
	var err error
	pathMap := map[string]string{
		"app_id": appId,
	}
	headerMap := map[string]string{}
	if o == nil {
		opts := EndpointMcpEventsUnsubscribeOptions{}
		o = &opts
	}
	internal.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
	if err != nil {
		return err
	}
	_, err = internal.ExecuteRequest[models.McpEventsUnsubscribeIn, any](
		ctx,
		endpointMcpEvents.client,
		"POST",
		"/api/v1/app/{app_id}/mcp-events/unsubscribe",
		pathMap,
		nil,
		headerMap,
		&mcpEventsUnsubscribeIn,
	)
	return err
}
