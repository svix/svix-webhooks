// Package svix this file is @generated DO NOT EDIT
package models

type RedshiftPatchConfig struct {
	AccessKeyId *string `json:"accessKeyId,omitempty"`
	// Database name.
	//
	// Only required if not using transformations.
	DbName *string `json:"dbName,omitempty"`
	Region *string `json:"region,omitempty"`
	// Schema name.
	//
	// Only used if not using transformations.
	SchemaName      *string `json:"schemaName,omitempty"`
	SecretAccessKey *string `json:"secretAccessKey,omitempty"`
	// Table name.
	//
	// Only required if not using transformations.
	TableName *string `json:"tableName,omitempty"`
}
