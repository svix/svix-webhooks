// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type EventBridgeConfigPatch struct {
	EventBusName    *string `json:"eventBusName,omitempty"`
	DetailType      *string `json:"detailType,omitempty"`
	AccessKeyId     *string `json:"accessKeyId,omitempty"`
	SecretAccessKey *string `json:"secretAccessKey,omitempty"`
	Region          *string `json:"region,omitempty"`
}

func (o EventBridgeConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.EventBusName != nil {
		toSerialize["eventBusName"] = o.EventBusName
	}
	if o.DetailType != nil {
		toSerialize["detailType"] = o.DetailType
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
	return json.Marshal(toSerialize)
}
