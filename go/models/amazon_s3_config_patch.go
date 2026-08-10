// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type AmazonS3ConfigPatch struct {
	Bucket          *string `json:"bucket,omitempty"`
	AccessKeyId     *string `json:"accessKeyId,omitempty"`
	SecretAccessKey *string `json:"secretAccessKey,omitempty"`
	Region          *string `json:"region,omitempty"`
	EndpointUrl     *string `json:"endpointUrl,omitempty"`
}

func (o AmazonS3ConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Bucket != nil {
		toSerialize["bucket"] = o.Bucket
	}
	if o.AccessKeyId != nil {
		toSerialize["accessKeyId"] = o.AccessKeyId
	}
	if o.SecretAccessKey != nil {
		toSerialize["secretAccessKey"] = o.SecretAccessKey
	}
	if o.Region != nil {
		toSerialize["region"] = o.Region
	}
	if o.EndpointUrl != nil {
		toSerialize["endpointUrl"] = o.EndpointUrl
	}
	return json.Marshal(toSerialize)
}
