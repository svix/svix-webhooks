// Package svix this file is @generated DO NOT EDIT
package models

type PollingEndpointOut struct {
	Data     []PollingEndpointMessageOut `json:"data"`
	Done     bool                        `json:"done"`
	Iterator string                      `json:"iterator"`
}
