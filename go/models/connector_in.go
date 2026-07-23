// Package svix this file is @generated DO NOT EDIT
package models

type ConnectorIn struct {
	Name              string            `json:"name"`
	Uid               *string           `json:"uid,omitempty"` // The Connector's UID.
	Logo              *string           `json:"logo,omitempty"`
	Description       *string           `json:"description,omitempty"`
	Kind              *ConnectorKind    `json:"kind,omitempty"`
	Instructions      *string           `json:"instructions,omitempty"`
	AllowedEventTypes []string          `json:"allowedEventTypes,omitempty"`
	Transformation    string            `json:"transformation"`
	FeatureFlags      []string          `json:"featureFlags,omitempty"`
	ProductType       *ConnectorProduct `json:"productType,omitempty"`
}
