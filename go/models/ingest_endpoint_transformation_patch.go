// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type IngestEndpointTransformationPatch struct {
	Code    utils.Nullable[string] `json:"code"`
	Enabled *bool                  `json:"enabled,omitempty"`
}

func (o IngestEndpointTransformationPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Code.IsSet() {
		toSerialize["code"] = o.Code
	}
	if o.Enabled != nil {
		toSerialize["enabled"] = o.Enabled
	}
	return json.Marshal(toSerialize)
}
