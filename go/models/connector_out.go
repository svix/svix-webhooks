// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ConnectorOut struct {
	CreatedAt        time.Time     `json:"createdAt"`
	Description      string        `json:"description"`
	FeatureFlag      *string       `json:"featureFlag,omitempty"`
	FilterTypes      []string      `json:"filterTypes,omitempty"`
	Id               string        `json:"id"` // The Connector's ID.
	Instructions     string        `json:"instructions"`
	InstructionsLink *string       `json:"instructionsLink,omitempty"`
	Kind             ConnectorKind `json:"kind"`
	Logo             string        `json:"logo"`
	Name             string        `json:"name"`
	OrgId            string        `json:"orgId"` // The Environment's ID.
	Transformation   string        `json:"transformation"`
	UpdatedAt        time.Time     `json:"updatedAt"`
}
