// Package svix this file is @generated DO NOT EDIT
package models

type EventBridgeConfigIn struct {
	EventBusName string  `json:"eventBusName"`         // The name or ARN of the event bus to receive the event
	DetailType   *string `json:"detailType,omitempty"` // Free-form string, with a maximum of 128 characters
	// Access key ID.
	//
	// Currently a required field, but marked as optional because we may add different authentication in the future.
	AccessKeyId *string `json:"accessKeyId,omitempty"`
	// Secret access key.
	//
	// Currently a required field, but marked as optional because we may add different authentication in the future.
	SecretAccessKey *string `json:"secretAccessKey,omitempty"`
	// The region of the EventBridge bus.
	//
	// Currently a required field, but marked as optional because we may infer it from other fields in the future.
	Region *string `json:"region,omitempty"`
}
