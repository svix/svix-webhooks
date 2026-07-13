// Package svix this file is @generated DO NOT EDIT
package models

type PollingEndpointOut struct {
	Data     []PollingEndpointMessageOut `json:"data"`
	Iterator string                      `json:"iterator"`
	Done     bool                        `json:"done"`
}
