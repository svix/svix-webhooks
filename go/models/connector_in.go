// Package svix this file is @generated DO NOT EDIT
package models

type ConnectorIn struct {
	AllowedEventTypes []string          `json:"allowedEventTypes,omitempty"`
	Description       *string           `json:"description,omitempty"`
	FeatureFlags      []string          `json:"featureFlags,omitempty"`
	Instructions      *string           `json:"instructions,omitempty"`
	Kind              *ConnectorKind    `json:"kind,omitempty"`
	Logo              *string           `json:"logo,omitempty"`
	Name              string            `json:"name"`
	ProductType       *ConnectorProduct `json:"productType,omitempty"`
	Transformation    string            `json:"transformation"`
	Uid               *string           `json:"uid,omitempty"` // The Connector's UID.
}
