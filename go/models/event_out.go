// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type EventOut struct {
	EventType string    `json:"eventType"` // The event type's name
	Payload   string    `json:"payload"`
	Timestamp time.Time `json:"timestamp"`
}
