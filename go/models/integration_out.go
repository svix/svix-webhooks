// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type IntegrationOut struct {
	CreatedAt    time.Time `json:"createdAt"`
	FeatureFlags []string  `json:"featureFlags,omitempty"` // The set of feature flags the integration has access to.
	Id           string    `json:"id"`                     // The Integration's ID.
	Name         string    `json:"name"`
	UpdatedAt    time.Time `json:"updatedAt"`
}
