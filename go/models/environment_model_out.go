// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type EnvironmentModelOut struct {
	Id        string            `json:"id"` // The Environment's ID.
	CreatedAt time.Time         `json:"createdAt"`
	UpdatedAt time.Time         `json:"updatedAt"`
	Region    EnvironmentRegion `json:"region"`
	Name      string            `json:"name"`
	Type      EnvironmentType   `json:"type"`
}
