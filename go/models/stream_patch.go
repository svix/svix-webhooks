// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type StreamPatch struct {
	Description *string                `json:"description,omitempty"` // The Stream's description.
	Uid         utils.Nullable[string] `json:"uid"`                   // An optional unique identifier for the stream.
	Metadata    *map[string]string     `json:"metadata,omitempty"`
}

func (o StreamPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Description != nil {
		toSerialize["description"] = o.Description
	}
	if o.Uid.IsSet() {
		toSerialize["uid"] = o.Uid
	}
	if o.Metadata != nil {
		toSerialize["metadata"] = o.Metadata
	}
	return json.Marshal(toSerialize)
}
