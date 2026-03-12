// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type ApiTokenPatch struct {
	Name   utils.Nullable[string]   `json:"name"`
	Scopes utils.Nullable[[]string] `json:"scopes"`
}

func (o ApiTokenPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Name.IsSet() {
		toSerialize["name"] = o.Name
	}
	if o.Scopes.IsSet() {
		toSerialize["scopes"] = o.Scopes
	}
	return json.Marshal(toSerialize)
}
