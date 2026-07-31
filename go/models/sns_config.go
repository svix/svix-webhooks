// Package svix this file is @generated DO NOT EDIT
package models

// Configuration for a SNS sink.
type SnsConfig struct {
	TopicArn        string  `json:"topicArn"`
	Region          string  `json:"region"`
	AccessKeyId     string  `json:"accessKeyId"`
	SecretAccessKey string  `json:"secretAccessKey"`
	EndpointUrl     *string `json:"endpointUrl,omitempty"`
}
