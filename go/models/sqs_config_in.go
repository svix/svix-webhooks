// Package svix this file is @generated DO NOT EDIT
package models

// Configuration for an SQS sink.
type SqsConfigIn struct {
	QueueUrl string `json:"queueUrl"`
	// The region of the SQS queue.
	//
	// Currently a required field, but marked as optional because we may infer it from other fields in the future.
	Region *string `json:"region,omitempty"`
	// Access key ID.
	//
	// Currently a required field, but marked as optional because we may add different authentication in the future.
	AccessKeyId *string `json:"accessKeyId,omitempty"`
	// Secret access key.
	//
	// Currently a required field, but marked as optional because we may add different authentication in the future.
	SecretAccessKey *string `json:"secretAccessKey,omitempty"`
	EndpointUrl     *string `json:"endpointUrl,omitempty"`
}
