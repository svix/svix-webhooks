// Package svix this file is @generated DO NOT EDIT
package models

type EventBridgeConfig struct {
	EventBusName    string  `json:"eventBusName"`         // The name or ARN of the event bus to receive the event
	DetailType      *string `json:"detailType,omitempty"` // Free-form string, with a maximum of 128 characters
	AccessKeyId     string  `json:"accessKeyId"`
	SecretAccessKey string  `json:"secretAccessKey"`
	Region          string  `json:"region"`
}
