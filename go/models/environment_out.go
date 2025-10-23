// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type EnvironmentOut struct {
	Connectors []ConnectorOut  `json:"connectors"`
	CreatedAt  time.Time       `json:"createdAt"`
	EventTypes []EventTypeOut  `json:"eventTypes"`
	Settings   *map[string]any `json:"settings,omitempty"`
	Version    *int64          `json:"version,omitempty"`
}
