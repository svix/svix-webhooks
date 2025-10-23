// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ConnectorOut struct {
	AllowedEventTypes []string      `json:"allowedEventTypes,omitempty"`
	CreatedAt         time.Time     `json:"createdAt"`
	Description       string        `json:"description"`
	FeatureFlags      []string      `json:"featureFlags,omitempty"`
	Id                string        `json:"id"` // The Connector's ID.
	Instructions      string        `json:"instructions"`
	Kind              ConnectorKind `json:"kind"`
	Logo              *string       `json:"logo,omitempty"`
	Name              string        `json:"name"`
	OrgId             string        `json:"orgId"` // The Environment's ID.
	Transformation    string        `json:"transformation"`
	UpdatedAt         time.Time     `json:"updatedAt"`
}
