// Package svix this file is @generated DO NOT EDIT
package models

type RedshiftConfigOut struct {
	AccessKeyId       string  `json:"accessKeyId"`
	Region            string  `json:"region"`
	ClusterIdentifier *string `json:"clusterIdentifier,omitempty"`
	DbUser            *string `json:"dbUser,omitempty"`
	WorkgroupName     *string `json:"workgroupName,omitempty"`
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
