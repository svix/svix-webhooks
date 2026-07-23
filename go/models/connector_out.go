// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ConnectorOut struct {
	Id                      string           `json:"id"`            // The Connector's ID.
	OrgId                   string           `json:"orgId"`         // The Environment's ID.
	Uid                     *string          `json:"uid,omitempty"` // The Connector's UID.
	Kind                    ConnectorKind    `json:"kind"`
	Name                    string           `json:"name"`
	Logo                    *string          `json:"logo,omitempty"`
	Description             string           `json:"description"`
	Instructions            string           `json:"instructions"`
	AllowedEventTypes       []string         `json:"allowedEventTypes,omitempty"`
	Transformation          string           `json:"transformation"`
	CreatedAt               time.Time        `json:"createdAt"`
	UpdatedAt               time.Time        `json:"updatedAt"`
	TransformationUpdatedAt time.Time        `json:"transformationUpdatedAt"`
	FeatureFlags            []string         `json:"featureFlags,omitempty"`
	ProductType             ConnectorProduct `json:"productType"`
}
