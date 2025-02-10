// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type MessageOut struct {
	// List of free-form identifiers that endpoints can filter by
	Channels []string `json:"channels,omitempty"`
	// Optional unique identifier for the message
	EventId *string `json:"eventId,omitempty"`
	// The event type's name
	EventType string `json:"eventType"`
	// The msg's ID
	Id        string                 `json:"id"`
	Payload   map[string]interface{} `json:"payload"`
	Tags      []string               `json:"tags,omitempty"`
	Timestamp time.Time              `json:"timestamp"`
}
