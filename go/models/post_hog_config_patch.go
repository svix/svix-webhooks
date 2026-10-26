// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type PostHogConfigPatch struct {
	Domain       *string `json:"domain,omitempty"`
	ProjectToken *string `json:"projectToken,omitempty"`
}

func (o PostHogConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Domain != nil {
		toSerialize["domain"] = o.Domain
	}
	if o.ProjectToken != nil {
		toSerialize["projectToken"] = o.ProjectToken
	}
	return json.Marshal(toSerialize)
}
