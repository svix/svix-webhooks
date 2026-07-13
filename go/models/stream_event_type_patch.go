// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type StreamEventTypePatch struct {
	Description  utils.Nullable[string]   `json:"description"`
	FeatureFlags utils.Nullable[[]string] `json:"featureFlags"`
	Deprecated   *bool                    `json:"deprecated,omitempty"`
	Archived     *bool                    `json:"archived,omitempty"`
}

func (o StreamEventTypePatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Description.IsSet() {
		toSerialize["description"] = o.Description
	}
	if o.FeatureFlags.IsSet() {
		toSerialize["featureFlags"] = o.FeatureFlags
	}
	if o.Deprecated != nil {
		toSerialize["deprecated"] = o.Deprecated
	}
	if o.Archived != nil {
		toSerialize["archived"] = o.Archived
	}
	return json.Marshal(toSerialize)
}
