// Package svix this file is @generated DO NOT EDIT
package models

type EnvironmentIn struct {
	Connectors []ConnectorIn   `json:"connectors,omitempty"`
	EventTypes []EventTypeIn   `json:"eventTypes,omitempty"`
	Settings   *map[string]any `json:"settings,omitempty"`
}
