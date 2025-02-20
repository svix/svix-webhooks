// Package svix this file is @generated DO NOT EDIT
package models

type IntegrationUpdate struct {
	FeatureFlags []string `json:"featureFlags,omitempty"` // The set of feature flags the integration will have access to.
	Name         string   `json:"name"`
}
