// Package svix this file is @generated DO NOT EDIT
package models

type ConnectorIn struct {
	Description    *string        `json:"description,omitempty"`
	FeatureFlags   []string       `json:"featureFlags,omitempty"`
	FilterTypes    []string       `json:"filterTypes,omitempty"`
	Instructions   *string        `json:"instructions,omitempty"`
	Kind           *ConnectorKind `json:"kind,omitempty"`
	Logo           string         `json:"logo"`
	Name           string         `json:"name"`
	Transformation string         `json:"transformation"`
}
