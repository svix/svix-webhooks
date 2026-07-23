// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type StreamEventTypeOut struct {
	Name         string    `json:"name"` // The event type's name
	Description  *string   `json:"description,omitempty"`
	CreatedAt    time.Time `json:"createdAt"`
	UpdatedAt    time.Time `json:"updatedAt"`
	Deprecated   bool      `json:"deprecated"`
	Archived     bool      `json:"archived"`
	FeatureFlags []string  `json:"featureFlags,omitempty"`
}
