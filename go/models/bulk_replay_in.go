// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type BulkReplayIn struct {
	Channel    *string        `json:"channel,omitempty"`
	EventTypes []string       `json:"eventTypes,omitempty"`
	Since      time.Time      `json:"since"`
	Status     *MessageStatus `json:"status,omitempty"`
	Tag        *string        `json:"tag,omitempty"`
	Until      *time.Time     `json:"until,omitempty"`
}
