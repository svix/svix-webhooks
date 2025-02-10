// Package svix this file is @generated DO NOT EDIT
package models

import "time"

// A model containing information on a given message plus additional fields on the last attempt for that message.
type EndpointMessageOut struct {
	// List of free-form identifiers that endpoints can filter by
	Channels []string `json:"channels,omitempty"`
	// Optional unique identifier for the message
	EventId *string `json:"eventId,omitempty"`
	// The event type's name
	EventType string `json:"eventType"`
	// The msg's ID
	Id          string                 `json:"id"`
	NextAttempt *time.Time             `json:"nextAttempt,omitempty"`
	Payload     map[string]interface{} `json:"payload"`
	Status      MessageStatus          `json:"status"`
	Tags        []string               `json:"tags,omitempty"`
	Timestamp   time.Time              `json:"timestamp"`
}
