// Package svix this file is @generated DO NOT EDIT
package models

type EventBridgeConfig struct {
	AccessKeyId     string  `json:"accessKeyId"`
	DetailType      *string `json:"detailType,omitempty"` // Free-form string, with a maximum of 128 characters
	EventBusName    string  `json:"eventBusName"`         // The name or ARN of the event bus to receive the event
	Region          string  `json:"region"`
	SecretAccessKey string  `json:"secretAccessKey"`
}
