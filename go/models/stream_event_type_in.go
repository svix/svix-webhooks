// Package svix this file is @generated DO NOT EDIT
package models

type StreamEventTypeIn struct {
	Archived     *bool    `json:"archived,omitempty"`
	Deprecated   *bool    `json:"deprecated,omitempty"`
	Description  *string  `json:"description,omitempty"`
	FeatureFlags []string `json:"featureFlags,omitempty"`
	Name         string   `json:"name"` // The event type's name
}
