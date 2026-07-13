// Package svix this file is @generated DO NOT EDIT
package models

type EventTypeFromOpenApi struct {
	Name         string          `json:"name"` // The event type's name
	Description  string          `json:"description"`
	Schemas      *map[string]any `json:"schemas,omitempty"`
	Deprecated   bool            `json:"deprecated"`
	GroupName    *string         `json:"groupName,omitempty"` // The event type group's name
	FeatureFlags []string        `json:"featureFlags,omitempty"`
}
