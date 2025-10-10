// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type StreamEventTypePatch struct {
	Archived     *bool                    `json:"archived,omitempty"`
	Deprecated   *bool                    `json:"deprecated,omitempty"`
	Description  utils.Nullable[string]   `json:"description"`
	FeatureFlags utils.Nullable[[]string] `json:"featureFlags"`
	Name         utils.Nullable[string]   `json:"name"` // The event type's name
}

func (o StreamEventTypePatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Archived != nil {
		toSerialize["archived"] = o.Archived
	}
	if o.Deprecated != nil {
		toSerialize["deprecated"] = o.Deprecated
	}
	if o.Description.IsSet() {
		toSerialize["description"] = o.Description
	}
	if o.FeatureFlags.IsSet() {
		toSerialize["featureFlags"] = o.FeatureFlags
	}
	if o.Name.IsSet() {
		toSerialize["name"] = o.Name
	}
	return json.Marshal(toSerialize)
}
