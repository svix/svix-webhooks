// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type HttpConfigPatch struct {
	Url *string `json:"url,omitempty"`
}

func (o HttpConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Url != nil {
		toSerialize["url"] = o.Url
	}
	return json.Marshal(toSerialize)
}
