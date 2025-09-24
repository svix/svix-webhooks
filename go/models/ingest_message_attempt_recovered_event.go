// Package svix this file is @generated DO NOT EDIT
package models

// Sent on a successful dispatch after an earlier failure op webhook has already been sent.
type IngestMessageAttemptRecoveredEvent struct {
	Data IngestMessageAttemptRecoveredEventData `json:"data"`
	Type string                                 `json:"type"`
}
