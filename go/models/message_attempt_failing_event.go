// Package svix this file is @generated DO NOT EDIT
package models

// Sent after a message has been failing for a few times.
// It's sent on the fourth failure. It complements `message.attempt.exhausted` which is sent after the last failure.
type MessageAttemptFailingEvent struct {
	Data MessageAttemptFailingEventData `json:"data"`
	Type string                         `json:"type"`
}
