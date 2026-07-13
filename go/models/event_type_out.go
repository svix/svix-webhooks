// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type EventTypeOut struct {
	Name         string          `json:"name"` // The event type's name
	Description  string          `json:"description"`
	Archived     *bool           `json:"archived,omitempty"`
	Deprecated   bool            `json:"deprecated"`
	Schemas      *map[string]any `json:"schemas,omitempty"` // The schema for the event type for a specific version as a JSON schema.
	CreatedAt    time.Time       `json:"createdAt"`
	UpdatedAt    time.Time       `json:"updatedAt"`
	GroupName    *string         `json:"groupName,omitempty"` // The event type group's name
	FeatureFlags []string        `json:"featureFlags,omitempty"`
	FeatureFlag  *string         `json:"featureFlag,omitempty"`
}
