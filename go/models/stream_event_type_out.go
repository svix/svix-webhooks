// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type StreamEventTypeOut struct {
	Archived     bool      `json:"archived"`
	CreatedAt    time.Time `json:"createdAt"`
	Deprecated   bool      `json:"deprecated"`
	Description  *string   `json:"description,omitempty"`
	FeatureFlags []string  `json:"featureFlags,omitempty"`
	Name         string    `json:"name"` // The event type's name
	UpdatedAt    time.Time `json:"updatedAt"`
}
