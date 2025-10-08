// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type StreamPatch struct {
	Description *string                `json:"description,omitempty"` // The Stream's description.
	Metadata    *map[string]string     `json:"metadata,omitempty"`
	Uid         utils.Nullable[string] `json:"uid"` // An optional unique identifier for the stream.
}

func (o StreamPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Description != nil {
		toSerialize["description"] = o.Description
	}
	if o.Metadata != nil {
		toSerialize["metadata"] = o.Metadata
	}
	if o.Uid.IsSet() {
		toSerialize["uid"] = o.Uid
	}
	return json.Marshal(toSerialize)
}
