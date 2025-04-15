// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type EnvironmentModelOut struct {
	CreatedAt time.Time         `json:"createdAt"`
	Id        string            `json:"id"` // The Environment's ID.
	Name      string            `json:"name"`
	Region    EnvironmentRegion `json:"region"`
	Type      EnvironmentType   `json:"type"`
	UpdatedAt time.Time         `json:"updatedAt"`
}
