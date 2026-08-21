// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type ClickhouseConfigPatch struct {
	Url       *string `json:"url,omitempty"`
	Username  *string `json:"username,omitempty"`
	Password  *string `json:"password,omitempty"`
	Database  *string `json:"database,omitempty"`
	TableName *string `json:"tableName,omitempty"`
}

func (o ClickhouseConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Url != nil {
		toSerialize["url"] = o.Url
	}
	if o.Username != nil {
		toSerialize["username"] = o.Username
	}
	if o.Password != nil {
		toSerialize["password"] = o.Password
	}
	if o.Database != nil {
		toSerialize["database"] = o.Database
	}
	if o.TableName != nil {
		toSerialize["tableName"] = o.TableName
	}
	return json.Marshal(toSerialize)
}
