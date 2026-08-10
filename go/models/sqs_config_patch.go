// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type SqsConfigPatch struct {
	QueueUrl        *string                `json:"queueUrl,omitempty"`
	Region          *string                `json:"region,omitempty"`
	AccessKeyId     *string                `json:"accessKeyId,omitempty"`
	SecretAccessKey *string                `json:"secretAccessKey,omitempty"`
	EndpointUrl     utils.Nullable[string] `json:"endpointUrl"`
}

func (o SqsConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.QueueUrl != nil {
		toSerialize["queueUrl"] = o.QueueUrl
	}
	if o.Region != nil {
		toSerialize["region"] = o.Region
	}
	if o.AccessKeyId != nil {
		toSerialize["accessKeyId"] = o.AccessKeyId
	}
	if o.SecretAccessKey != nil {
		toSerialize["secretAccessKey"] = o.SecretAccessKey
	}
	if o.EndpointUrl.IsSet() {
		toSerialize["endpointUrl"] = o.EndpointUrl
	}
	return json.Marshal(toSerialize)
}
