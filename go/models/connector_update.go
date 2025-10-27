// Package svix this file is @generated DO NOT EDIT
package models

type ConnectorUpdate struct {
	AllowedEventTypes []string       `json:"allowedEventTypes,omitempty"`
	Description       *string        `json:"description,omitempty"`
	FeatureFlags      []string       `json:"featureFlags,omitempty"`
	Instructions      *string        `json:"instructions,omitempty"`
	Kind              *ConnectorKind `json:"kind,omitempty"`
	Logo              *string        `json:"logo,omitempty"`
	Name              *string        `json:"name,omitempty"`
	Transformation    string         `json:"transformation"`
}
