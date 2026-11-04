// Package svix this file is @generated DO NOT EDIT
package models

type EventBridgeConfigOut struct {
	EventBusName string  `json:"eventBusName"`
	DetailType   string  `json:"detailType"`
	AccessKeyId  *string `json:"accessKeyId,omitempty"`
	RoleArn      *string `json:"roleArn,omitempty"`
	ExternalId   *string `json:"externalId,omitempty"`
	Region       string  `json:"region"`
}
