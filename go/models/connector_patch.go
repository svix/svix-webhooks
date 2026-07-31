// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type ConnectorPatch struct {
	Name              *string                  `json:"name,omitempty"`
	Logo              utils.Nullable[string]   `json:"logo"`
	Description       *string                  `json:"description,omitempty"`
	Kind              *ConnectorKind           `json:"kind,omitempty"`
	Instructions      *string                  `json:"instructions,omitempty"`
	AllowedEventTypes utils.Nullable[[]string] `json:"allowedEventTypes"`
	Transformation    *string                  `json:"transformation,omitempty"`
	FeatureFlags      utils.Nullable[[]string] `json:"featureFlags"`
}

func (o ConnectorPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Name != nil {
		toSerialize["name"] = o.Name
	}
	if o.Logo.IsSet() {
		toSerialize["logo"] = o.Logo
	}
	if o.Description != nil {
		toSerialize["description"] = o.Description
	}
	if o.Kind != nil {
		toSerialize["kind"] = o.Kind
	}
	if o.Instructions != nil {
		toSerialize["instructions"] = o.Instructions
	}
	if o.AllowedEventTypes.IsSet() {
		toSerialize["allowedEventTypes"] = o.AllowedEventTypes
	}
	if o.Transformation != nil {
		toSerialize["transformation"] = o.Transformation
	}
	if o.FeatureFlags.IsSet() {
		toSerialize["featureFlags"] = o.FeatureFlags
	}
	return json.Marshal(toSerialize)
}
