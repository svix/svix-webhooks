// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type MessageAttemptFailedData struct {
	Id                 string    `json:"id"` // The MessageAttempt's ID.
	ResponseStatusCode int16     `json:"responseStatusCode"`
	Timestamp          time.Time `json:"timestamp"`
}
