// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type ConnectorPatch struct {
	AllowedEventTypes utils.Nullable[[]string] `json:"allowedEventTypes"`
	Description       *string                  `json:"description,omitempty"`
	FeatureFlags      utils.Nullable[[]string] `json:"featureFlags"`
	Instructions      *string                  `json:"instructions,omitempty"`
	Kind              *ConnectorKind           `json:"kind,omitempty"`
	Logo              utils.Nullable[string]   `json:"logo"`
	Name              *string                  `json:"name,omitempty"`
	Transformation    *string                  `json:"transformation,omitempty"`
}

func (o ConnectorPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.AllowedEventTypes.IsSet() {
		toSerialize["allowedEventTypes"] = o.AllowedEventTypes
	}
	if o.Description != nil {
		toSerialize["description"] = o.Description
	}
	if o.FeatureFlags.IsSet() {
		toSerialize["featureFlags"] = o.FeatureFlags
	}
	if o.Instructions != nil {
		toSerialize["instructions"] = o.Instructions
	}
	if o.Kind != nil {
		toSerialize["kind"] = o.Kind
	}
	if o.Logo.IsSet() {
		toSerialize["logo"] = o.Logo
	}
	if o.Name != nil {
		toSerialize["name"] = o.Name
	}
	if o.Transformation != nil {
		toSerialize["transformation"] = o.Transformation
	}
	return json.Marshal(toSerialize)
}
