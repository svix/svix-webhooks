// Package svix this file is @generated DO NOT EDIT
package models

type McpEventsUnsubscribeDeliveryIn struct {
	Mode McpEventsDeliveryMode `json:"mode"` // How events are delivered. Only `webhook` is supported.
	Url  string                `json:"url"`  // The callback URL the subscription was made with.
}
