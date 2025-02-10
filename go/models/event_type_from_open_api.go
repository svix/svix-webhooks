// Package svix this file is @generated DO NOT EDIT
package models

type EventTypeFromOpenApi struct {
	Deprecated  bool    `json:"deprecated"`
	Description string  `json:"description"`
	FeatureFlag *string `json:"featureFlag,omitempty"`
	// The event type group's name
	GroupName *string `json:"groupName,omitempty"`
	// The event type's name
	Name    string                  `json:"name"`
	Schemas *map[string]interface{} `json:"schemas,omitempty"`
}
