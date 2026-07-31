// Package svix this file is @generated DO NOT EDIT
package models

type MessagePrecheckIn struct {
	EventType string   `json:"eventType"` // The event type's name
	Channels  []string `json:"channels,omitempty"`
}
