// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type McpEventsSubscribeOut struct {
	Id            string     `json:"id"`                      // The subscription's ID, which is also the UID of its endpoint.
	EndpointId    string     `json:"endpointId"`              // The Endpoint's ID.
	RefreshBefore *time.Time `json:"refreshBefore,omitempty"` // When the subscription expires, or `null` if it never does.
	Cursor        *string    `json:"cursor,omitempty"`        // Always `null`, since events can't be replayed.
	Truncated     bool       `json:"truncated"`               // Always `false`, since events can't be replayed.
}
