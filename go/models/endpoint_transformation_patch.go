// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type EndpointTransformationPatch struct {
	Code      utils.Nullable[string]            `json:"code"`
	Enabled   *bool                             `json:"enabled,omitempty"`
	Variables utils.Nullable[map[string]string] `json:"variables"`
}

func (o EndpointTransformationPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Code.IsSet() {
		toSerialize["code"] = o.Code
	}
	if o.Enabled != nil {
		toSerialize["enabled"] = o.Enabled
	}
	if o.Variables.IsSet() {
		toSerialize["variables"] = o.Variables
	}
	return json.Marshal(toSerialize)
}
