// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type RabbitMqConfigPatch struct {
	RoutingKey *string `json:"routingKey,omitempty"`
	Uri        *string `json:"uri,omitempty"`
}

func (o RabbitMqConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.RoutingKey != nil {
		toSerialize["routingKey"] = o.RoutingKey
	}
	if o.Uri != nil {
		toSerialize["uri"] = o.Uri
	}
	return json.Marshal(toSerialize)
}
