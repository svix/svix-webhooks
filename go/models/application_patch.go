// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type ApplicationPatch struct {
	Name *string `json:"name,omitempty"`
	// Maximum messages per second to send to this application.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate utils.Nullable[uint16] `json:"throttleRate"`
	Uid          utils.Nullable[string] `json:"uid"` // The Application's UID.
	Metadata     *map[string]string     `json:"metadata,omitempty"`
}

func (o ApplicationPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Name != nil {
		toSerialize["name"] = o.Name
	}
	if o.ThrottleRate.IsSet() {
		toSerialize["throttleRate"] = o.ThrottleRate
	}
	if o.Uid.IsSet() {
		toSerialize["uid"] = o.Uid
	}
	if o.Metadata != nil {
		toSerialize["metadata"] = o.Metadata
	}
	return json.Marshal(toSerialize)
}
