// Package svix this file is @generated DO NOT EDIT
package models

// Configuration parameters for defining a Redshift sink.
//
// For provisioned clusters, set `cluster_identifier` and `db_user`. For Redshift Serverless, set `workgroup_name`.
type RedshiftConfigIn struct {
	// Access key ID.
	//
	// Required (along with `secret_access_key`) if `role_arn` is blank.
	AccessKeyId *string `json:"accessKeyId,omitempty"`
	// Secret access key.
	//
	// Required (along with `access_key_id`) if `role_arn` is blank.
	SecretAccessKey *string `json:"secretAccessKey,omitempty"`
	RoleArn         *string `json:"roleArn,omitempty"` // Role ARN for delegated authentication
	// Shared secret passed as the STS ExternalId.
	//
	// Can only be set if `role_arn` is Some
	ExternalId *string `json:"externalId,omitempty"`
	// The region of the Redshift DB.
	//
	// Currently a required field, but marked as optional because we may infer it from other fields in the future.
	Region            *string `json:"region,omitempty"`
	ClusterIdentifier *string `json:"clusterIdentifier,omitempty"` // Required for provisioned clusters.
	DbUser            *string `json:"dbUser,omitempty"`            // Required for provisioned clusters.
	WorkgroupName     *string `json:"workgroupName,omitempty"`     // Required for Redshift Serverless.
	// Database name.
	//
	// Only required if not using transformations.
	DbName *string `json:"dbName,omitempty"`
	// Schema name.
	//
	// Only used if not using transformations.
	SchemaName *string `json:"schemaName,omitempty"`
	// Table name.
	//
	// Only required if not using transformations.
	TableName *string `json:"tableName,omitempty"`
}
