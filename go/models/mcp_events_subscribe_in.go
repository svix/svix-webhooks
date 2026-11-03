// Package svix this file is @generated DO NOT EDIT
package models

type McpEventsSubscribeIn struct {
	Name string `json:"name"` // The event to subscribe to. It must be an existing event type.
	// The filter arguments of the `events/subscribe` request, as described by
	// the event's `inputSchema` (e.g. `{"document_id": "doc_123"}`).
	// Together with `name` and the callback URL they identify the subscription
	//
	// They're stored in the endpoint's `mcpEventArguments` metadata, but
	// aren't used to filter messages; use `channels` for that.
	//
	// See [Create a subscription](https://developers.openai.com/plugins/build/mcp-events#create-a-subscription)
	// in OpenAI's MCP Events docs.
	Arguments *map[string]any              `json:"arguments,omitempty"`
	Delivery  McpEventsSubscribeDeliveryIn `json:"delivery"`
	// The requested lifetime of the subscription, in milliseconds. Omit for
	// the default of 7 days, or set to `null` for a subscription that never
	// expires.
	TtlMs *uint64 `json:"ttlMs,omitempty"`
	// How long the previous secret keeps signing when a refresh changes it,
	// in seconds.
	//
	// Valid values are between 0 (immediate expiry) and 7 days. The default
	// is 5 minutes.
	SecretGracePeriodSeconds *uint32                `json:"secretGracePeriodSeconds,omitempty"`
	ChallengeMode            *EndpointChallengeMode `json:"challengeMode,omitempty"` // How the callback must respond to the verification challenge.
	// Only send messages with one of these channels to the subscription,
	// for example ones derived from `arguments`. Omit to send all messages
	// of the event type.
	Channels []string `json:"channels,omitempty"`
}
