// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type SegmentSinkConfigPatch struct {
	Host     *string `json:"host,omitempty"`
	WriteKey *string `json:"writeKey,omitempty"`
}

func (o SegmentSinkConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Host != nil {
		toSerialize["host"] = o.Host
	}
	if o.WriteKey != nil {
		toSerialize["writeKey"] = o.WriteKey
	}
	return json.Marshal(toSerialize)
}
