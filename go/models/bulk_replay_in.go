// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type BulkReplayIn struct {
	Since           time.Time        `json:"since"`
	Until           *time.Time       `json:"until,omitempty"`
	EventTypes      []string         `json:"eventTypes,omitempty"`
	Channel         *string          `json:"channel,omitempty"`
	Tag             *string          `json:"tag,omitempty"`
	Status          *MessageStatus   `json:"status,omitempty"`
	StatusCodeClass *StatusCodeClass `json:"statusCodeClass,omitempty"`
}
