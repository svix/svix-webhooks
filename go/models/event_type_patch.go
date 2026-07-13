// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type EventTypePatch struct {
	Description  *string                        `json:"description,omitempty"`
	Archived     *bool                          `json:"archived,omitempty"`
	Deprecated   *bool                          `json:"deprecated,omitempty"`
	Schemas      utils.Nullable[map[string]any] `json:"schemas"`
	FeatureFlags utils.Nullable[[]string]       `json:"featureFlags"`
	GroupName    utils.Nullable[string]         `json:"groupName"` // The event type group's name
}

func (o EventTypePatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Description != nil {
		toSerialize["description"] = o.Description
	}
	if o.Archived != nil {
		toSerialize["archived"] = o.Archived
	}
	if o.Deprecated != nil {
		toSerialize["deprecated"] = o.Deprecated
	}
	if o.Schemas.IsSet() {
		toSerialize["schemas"] = o.Schemas
	}
	if o.FeatureFlags.IsSet() {
		toSerialize["featureFlags"] = o.FeatureFlags
	}
	if o.GroupName.IsSet() {
		toSerialize["groupName"] = o.GroupName
	}
	return json.Marshal(toSerialize)
}
