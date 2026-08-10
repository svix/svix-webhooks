// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type RedshiftConfigPatch struct {
	AccessKeyId     *string `json:"accessKeyId,omitempty"`
	SecretAccessKey *string `json:"secretAccessKey,omitempty"`
	Region          *string `json:"region,omitempty"`
	// Database name.
	//
	// Only required if not using transformations.
	DbName *string `json:"dbName,omitempty"`
	// Schema name.
	//
	// Only used if not using transformations.
	SchemaName utils.Nullable[string] `json:"schemaName"`
	// Table name.
	//
	// Only required if not using transformations.
	TableName *string `json:"tableName,omitempty"`
}

func (o RedshiftConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.AccessKeyId != nil {
		toSerialize["accessKeyId"] = o.AccessKeyId
	}
	if o.SecretAccessKey != nil {
		toSerialize["secretAccessKey"] = o.SecretAccessKey
	}
	if o.Region != nil {
		toSerialize["region"] = o.Region
	}
	if o.DbName != nil {
		toSerialize["dbName"] = o.DbName
	}
	if o.SchemaName.IsSet() {
		toSerialize["schemaName"] = o.SchemaName
	}
	if o.TableName != nil {
		toSerialize["tableName"] = o.TableName
	}
	return json.Marshal(toSerialize)
}
