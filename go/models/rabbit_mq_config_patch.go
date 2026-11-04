// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type RabbitMqConfigPatch struct {
	RoutingKey *string `json:"routingKey,omitempty"`
	Uri        *string `json:"uri,omitempty"`
	Mandatory  *bool   `json:"mandatory,omitempty"`
}

func (o RabbitMqConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.RoutingKey != nil {
		toSerialize["routingKey"] = o.RoutingKey
	}
	if o.Uri != nil {
		toSerialize["uri"] = o.Uri
	}
	if o.Mandatory != nil {
		toSerialize["mandatory"] = o.Mandatory
	}
	return json.Marshal(toSerialize)
}
