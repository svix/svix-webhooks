// Package svix this file is @generated DO NOT EDIT
package models

type MessagePrecheckIn struct {
	Channels  []string `json:"channels,omitempty"`
	EventType string   `json:"eventType"` // The event type's name
}
