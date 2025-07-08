// Package svix this file is @generated DO NOT EDIT
package models

type ConnectorIn struct {
	Description      *string        `json:"description,omitempty"`
	FeatureFlag      *string        `json:"featureFlag,omitempty"` // Deprecated - prefer featureFlags instead.
	FeatureFlags     []string       `json:"featureFlags,omitempty"`
	FilterTypes      []string       `json:"filterTypes,omitempty"`
	Instructions     *string        `json:"instructions,omitempty"`
	InstructionsLink *string        `json:"instructionsLink,omitempty"`
	Kind             *ConnectorKind `json:"kind,omitempty"`
	Logo             string         `json:"logo"`
	Name             string         `json:"name"`
	Transformation   string         `json:"transformation"`
}
