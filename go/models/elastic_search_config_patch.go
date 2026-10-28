// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type ElasticSearchConfigPatch struct {
	IndexName *string                 `json:"indexName,omitempty"`
	Url       *string                 `json:"url,omitempty"`
	ApiKey    *string                 `json:"apiKey,omitempty"`
	Refresh   *bool                   `json:"refresh,omitempty"`
	Operation *ElasticSearchOperation `json:"operation,omitempty"`
}

func (o ElasticSearchConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.IndexName != nil {
		toSerialize["indexName"] = o.IndexName
	}
	if o.Url != nil {
		toSerialize["url"] = o.Url
	}
	if o.ApiKey != nil {
		toSerialize["apiKey"] = o.ApiKey
	}
	if o.Refresh != nil {
		toSerialize["refresh"] = o.Refresh
	}
	if o.Operation != nil {
		toSerialize["operation"] = o.Operation
	}
	return json.Marshal(toSerialize)
}
