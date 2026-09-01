// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type PostgresConfigPatch struct {
	Url         *string `json:"url,omitempty"`
	Password    *string `json:"password,omitempty"`
	TableName   *string `json:"tableName,omitempty"`
	SslRootCert *string `json:"sslRootCert,omitempty"`
}

func (o PostgresConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Url != nil {
		toSerialize["url"] = o.Url
	}
	if o.Password != nil {
		toSerialize["password"] = o.Password
	}
	if o.TableName != nil {
		toSerialize["tableName"] = o.TableName
	}
	if o.SslRootCert != nil {
		toSerialize["sslRootCert"] = o.SslRootCert
	}
	return json.Marshal(toSerialize)
}
