// Package svix this file is @generated DO NOT EDIT
package models

// Sent after message attempts are made. Contains metadata about message attempts and their results. In order to reduce the frequency of webhooks, these are sent in batches periodically.
type MessageAttemptLogEvent struct {
	Data []MessageAttemptLog `json:"data"`
	Type string              `json:"type"`
}
