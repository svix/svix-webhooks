// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type IntegrationOut struct {
	Name         string    `json:"name"`
	Id           string    `json:"id"` // The Integration's ID.
	CreatedAt    time.Time `json:"createdAt"`
	UpdatedAt    time.Time `json:"updatedAt"`
	FeatureFlags []string  `json:"featureFlags,omitempty"` // The set of feature flags the integration has access to.
}
