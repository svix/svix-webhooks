// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type IntegrationOut struct {
	CreatedAt time.Time `json:"createdAt"`
	// The integ's ID
	Id        string    `json:"id"`
	Name      string    `json:"name"`
	UpdatedAt time.Time `json:"updatedAt"`
}
