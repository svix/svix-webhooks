// Package svix this file is @generated DO NOT EDIT
package models

import "time"

// A model containing information on a given message plus additional fields on the last attempt for that message.
type EndpointMessageOut struct {
	Status      MessageStatus     `json:"status"`
	StatusText  MessageStatusText `json:"statusText"`
	NextAttempt *time.Time        `json:"nextAttempt,omitempty"`
	EventId     *string           `json:"eventId,omitempty"` // Optional unique identifier for the message
	EventType   string            `json:"eventType"`         // The event type's name
	Payload     map[string]any    `json:"payload"`
	Channels    []string          `json:"channels,omitempty"` // List of free-form identifiers that endpoints can filter by
	Id          string            `json:"id"`                 // The Message's ID.
	Timestamp   time.Time         `json:"timestamp"`
	Tags        []string          `json:"tags,omitempty"`
	DeliverAt   *time.Time        `json:"deliverAt,omitempty"`
}
