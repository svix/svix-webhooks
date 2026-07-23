// Package svix this file is @generated DO NOT EDIT
package models

type ConnectorUpsertIn struct {
	Name              *string        `json:"name,omitempty"`
	Logo              *string        `json:"logo,omitempty"`
	Description       *string        `json:"description,omitempty"`
	Kind              *ConnectorKind `json:"kind,omitempty"`
	Instructions      *string        `json:"instructions,omitempty"`
	AllowedEventTypes []string       `json:"allowedEventTypes,omitempty"`
	Transformation    string         `json:"transformation"`
	FeatureFlags      []string       `json:"featureFlags,omitempty"`
}
