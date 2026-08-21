// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type AzureBlobStorageConfigPatch struct {
	Container *string `json:"container,omitempty"`
	Account   *string `json:"account,omitempty"`
	AccessKey *string `json:"accessKey,omitempty"`
}

func (o AzureBlobStorageConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Container != nil {
		toSerialize["container"] = o.Container
	}
	if o.Account != nil {
		toSerialize["account"] = o.Account
	}
	if o.AccessKey != nil {
		toSerialize["accessKey"] = o.AccessKey
	}
	return json.Marshal(toSerialize)
}
