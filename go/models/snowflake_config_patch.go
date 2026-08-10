// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type SnowflakeConfigPatch struct {
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

func (o SnowflakeConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.PrivateKey != nil {
		toSerialize["privateKey"] = o.PrivateKey
	}
	if o.AccountIdentifier != nil {
		toSerialize["accountIdentifier"] = o.AccountIdentifier
	}
	if o.UserId != nil {
		toSerialize["userId"] = o.UserId
	}
	if o.DbName != nil {
		toSerialize["dbName"] = o.DbName
	}
	if o.SchemaName != nil {
		toSerialize["schemaName"] = o.SchemaName
	}
	if o.TableName != nil {
		toSerialize["tableName"] = o.TableName
	}
	return json.Marshal(toSerialize)
}
