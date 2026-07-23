// Package svix this file is @generated DO NOT EDIT
package models

type EndpointStats struct {
	Success  int64 `json:"success"`
	Pending  int64 `json:"pending"`
	Sending  int64 `json:"sending"`
	Fail     int64 `json:"fail"`
	Canceled int64 `json:"canceled"`
}
