// Package svix this file is @generated DO NOT EDIT
package models

type McpEventsUnsubscribeIn struct {
	Name string `json:"name"` // The event the subscription was made for.
	// The filter arguments the subscription was made with, as given to
	// `events/subscribe`. See [Stop a subscription](https://developers.openai.com/plugins/build/mcp-events#stop-a-subscription)
	// in OpenAI's MCP Events docs.
	Arguments *map[string]any                `json:"arguments,omitempty"`
	Delivery  McpEventsUnsubscribeDeliveryIn `json:"delivery"`
}
