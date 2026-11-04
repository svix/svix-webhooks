// Package svix this file is @generated DO NOT EDIT
package models

type SqsConfigOut struct {
	QueueUrl    string  `json:"queueUrl"`
	Region      string  `json:"region"`
	AccessKeyId *string `json:"accessKeyId,omitempty"`
	RoleArn     *string `json:"roleArn,omitempty"`
	ExternalId  *string `json:"externalId,omitempty"`
	EndpointUrl *string `json:"endpointUrl,omitempty"`
}
