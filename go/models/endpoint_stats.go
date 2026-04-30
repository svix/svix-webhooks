// Package svix this file is @generated DO NOT EDIT
package models

type EndpointStats struct {
	Canceled int64 `json:"canceled"`
	Fail     int64 `json:"fail"`
	Pending  int64 `json:"pending"`
	Sending  int64 `json:"sending"`
	Success  int64 `json:"success"`
}
