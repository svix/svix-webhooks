// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type EndpointPatch struct {
	Channels    utils.Nullable[[]string] `json:"channels"`
	Description *string                  `json:"description,omitempty"`
	Disabled    *bool                    `json:"disabled,omitempty"`
	FilterTypes utils.Nullable[[]string] `json:"filterTypes"`
	Metadata    *map[string]string       `json:"metadata,omitempty"`
	// Maximum messages per second to send to this endpoint.
	//
	// Outgoing messages will be throttled to this rate.
	ThrottleRate utils.Nullable[uint16] `json:"throttleRate"`
	Uid          utils.Nullable[string] `json:"uid"` // The Endpoint's UID.
	Url          *string                `json:"url,omitempty"`
}

func (o EndpointPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Channels.IsSet() {
		toSerialize["channels"] = o.Channels
	}
	if o.Description != nil {
		toSerialize["description"] = o.Description
	}
	if o.Disabled != nil {
		toSerialize["disabled"] = o.Disabled
	}
	if o.FilterTypes.IsSet() {
		toSerialize["filterTypes"] = o.FilterTypes
	}
	if o.Metadata != nil {
		toSerialize["metadata"] = o.Metadata
	}
	if o.ThrottleRate.IsSet() {
		toSerialize["throttleRate"] = o.ThrottleRate
	}
	if o.Uid.IsSet() {
		toSerialize["uid"] = o.Uid
	}
	if o.Url != nil {
		toSerialize["url"] = o.Url
	}
	return json.Marshal(toSerialize)
}
