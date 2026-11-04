// Package svix this file is @generated DO NOT EDIT
package models

type SnsConfigOut struct {
	TopicArn    string  `json:"topicArn"`
	Region      string  `json:"region"`
	AccessKeyId *string `json:"accessKeyId,omitempty"`
	RoleArn     *string `json:"roleArn,omitempty"`
	ExternalId  *string `json:"externalId,omitempty"`
}
