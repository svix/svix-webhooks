// Package svix this file is @generated DO NOT EDIT
package models

// Configuration for a SNS sink.
type SnsConfigIn struct {
	TopicArn string `json:"topicArn"`
	// The region of the SNS instance.
	//
	// Currently a required field, but marked as optional because we may infer it from other fields in the future.
	Region *string `json:"region,omitempty"`
	// Access key ID.
	//
	// Required (along with `secret_access_key`) if `role_arn` is None
	AccessKeyId *string `json:"accessKeyId,omitempty"`
	// Secret access key.
	//
	// Required (along with `access_key_id`) if `role_arn` is None
	SecretAccessKey *string `json:"secretAccessKey,omitempty"`
	EndpointUrl     *string `json:"endpointUrl,omitempty"`
	RoleArn         *string `json:"roleArn,omitempty"` // Role ARN for delegated authentication
	// Shared secret passed as the STS ExternalId.
	//
	// Can only be set if `role_arn` is Some
	ExternalId *string `json:"externalId,omitempty"`
}
