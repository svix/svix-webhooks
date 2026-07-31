// Package svix this file is @generated DO NOT EDIT
package models

type StreamEventTypeIn struct {
	Name         string   `json:"name"` // The event type's name
	Description  *string  `json:"description,omitempty"`
	FeatureFlags []string `json:"featureFlags,omitempty"`
	Deprecated   *bool    `json:"deprecated,omitempty"`
	Archived     *bool    `json:"archived,omitempty"`
}
