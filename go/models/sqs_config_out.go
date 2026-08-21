// Package svix this file is @generated DO NOT EDIT
package models

type SqsConfigOut struct {
	QueueUrl    string  `json:"queueUrl"`
	Region      string  `json:"region"`
	AccessKeyId string  `json:"accessKeyId"`
	EndpointUrl *string `json:"endpointUrl,omitempty"`
}
