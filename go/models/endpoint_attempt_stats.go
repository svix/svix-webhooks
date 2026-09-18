// Package svix this file is @generated DO NOT EDIT
package models

type EndpointAttemptStats struct {
	Success  uint64 `json:"success"`
	Fail     uint64 `json:"fail"`
	Canceled uint64 `json:"canceled"`
}
