// Package svix this file is @generated DO NOT EDIT
package models

type EventTypeIn struct {
	Archived    *bool   `json:"archived,omitempty"`
	Deprecated  *bool   `json:"deprecated,omitempty"`
	Description string  `json:"description"`
	FeatureFlag *string `json:"featureFlag,omitempty"`
	// The event type group's name
	GroupName *string `json:"groupName,omitempty"`
	// The event type's name
	Name string `json:"name"`
	// The schema for the event type for a specific version as a JSON schema.
	Schemas *map[string]interface{} `json:"schemas,omitempty"`
}
