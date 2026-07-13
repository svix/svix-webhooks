// Package svix this file is @generated DO NOT EDIT
package models

type SnowflakePatchConfig struct {
	PrivateKey        *string `json:"privateKey,omitempty"`
	AccountIdentifier *string `json:"accountIdentifier,omitempty"`
	UserId            *string `json:"userId,omitempty"`
	// Database name.
	//
	// Only required if not using transformations.
	DbName *string `json:"dbName,omitempty"`
	// Schema name.
	//
	// Only required if not using transformations.
	SchemaName *string `json:"schemaName,omitempty"`
	// Table name.
	//
	// Only required if not using transformations.
	TableName *string `json:"tableName,omitempty"`
}
