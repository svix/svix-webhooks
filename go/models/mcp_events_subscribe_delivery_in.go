// Package svix this file is @generated DO NOT EDIT
package models

type McpEventsSubscribeDeliveryIn struct {
	Mode McpEventsDeliveryMode `json:"mode"` // How events are delivered. Only `webhook` is supported.
	Url  string                `json:"url"`  // The callback URL events are sent to.
	// The secret events are signed with. MCP Events requires a `whsec_`
	// secret.
	Secret string `json:"secret"`
}
