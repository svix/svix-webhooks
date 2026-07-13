// Package svix this file is @generated DO NOT EDIT
package models

// Configuration parameters for defining a Redshift sink.
//
// For provisioned clusters, set `cluster_identifier` and `db_user`. For Redshift Serverless, set `workgroup_name`.
type RedshiftConfig struct {
	AccessKeyId       string  `json:"accessKeyId"`
	SecretAccessKey   string  `json:"secretAccessKey"`
	Region            string  `json:"region"`
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
