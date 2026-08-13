// Package svix this file is @generated DO NOT EDIT
package models

// Configuration parameters for defining a Redshift sink.
//
// For provisioned clusters, set `cluster_identifier` and `db_user`. For Redshift Serverless, set `workgroup_name`.
type RedshiftConfigIn struct {
	// Access key ID.
	//
	// Currently a required field, but marked as optional because we may add different authentication in the future.
	AccessKeyId *string `json:"accessKeyId,omitempty"`
	// Secret access key.
	//
	// Currently a required field, but marked as optional because we may add different authentication in the future.
	SecretAccessKey *string `json:"secretAccessKey,omitempty"`
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
