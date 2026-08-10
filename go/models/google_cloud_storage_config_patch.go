// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type GoogleCloudStorageConfigPatch struct {
	Bucket      *string `json:"bucket,omitempty"`
	Credentials *string `json:"credentials,omitempty"`
}

func (o GoogleCloudStorageConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Bucket != nil {
		toSerialize["bucket"] = o.Bucket
	}
	if o.Credentials != nil {
		toSerialize["credentials"] = o.Credentials
	}
	return json.Marshal(toSerialize)
}
