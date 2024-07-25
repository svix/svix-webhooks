/*
 * Svix API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * API version: 1.1.1
 */

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
)

// MessageSubscriberAuthTokenOut struct for MessageSubscriberAuthTokenOut
type MessageSubscriberAuthTokenOut struct {
	BridgeToken string `json:"bridgeToken"`
	Token string `json:"token"`
}

// NewMessageSubscriberAuthTokenOut instantiates a new MessageSubscriberAuthTokenOut object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewMessageSubscriberAuthTokenOut(bridgeToken string, token string) *MessageSubscriberAuthTokenOut {
	this := MessageSubscriberAuthTokenOut{}
	this.BridgeToken = bridgeToken
	this.Token = token
	return &this
}

// NewMessageSubscriberAuthTokenOutWithDefaults instantiates a new MessageSubscriberAuthTokenOut object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewMessageSubscriberAuthTokenOutWithDefaults() *MessageSubscriberAuthTokenOut {
	this := MessageSubscriberAuthTokenOut{}
	return &this
}

// GetBridgeToken returns the BridgeToken field value
func (o *MessageSubscriberAuthTokenOut) GetBridgeToken() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.BridgeToken
}

// GetBridgeTokenOk returns a tuple with the BridgeToken field value
// and a boolean to check if the value has been set.
func (o *MessageSubscriberAuthTokenOut) GetBridgeTokenOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.BridgeToken, true
}

// SetBridgeToken sets field value
func (o *MessageSubscriberAuthTokenOut) SetBridgeToken(v string) {
	o.BridgeToken = v
}

// GetToken returns the Token field value
func (o *MessageSubscriberAuthTokenOut) GetToken() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Token
}

// GetTokenOk returns a tuple with the Token field value
// and a boolean to check if the value has been set.
func (o *MessageSubscriberAuthTokenOut) GetTokenOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.Token, true
}

// SetToken sets field value
func (o *MessageSubscriberAuthTokenOut) SetToken(v string) {
	o.Token = v
}

func (o MessageSubscriberAuthTokenOut) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if true {
		toSerialize["bridgeToken"] = o.BridgeToken
	}
	if true {
		toSerialize["token"] = o.Token
	}
	return json.Marshal(toSerialize)
}

type NullableMessageSubscriberAuthTokenOut struct {
	value *MessageSubscriberAuthTokenOut
	isSet bool
}

func (v NullableMessageSubscriberAuthTokenOut) Get() *MessageSubscriberAuthTokenOut {
	return v.value
}

func (v *NullableMessageSubscriberAuthTokenOut) Set(val *MessageSubscriberAuthTokenOut) {
	v.value = val
	v.isSet = true
}

func (v NullableMessageSubscriberAuthTokenOut) IsSet() bool {
	return v.isSet
}

func (v *NullableMessageSubscriberAuthTokenOut) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableMessageSubscriberAuthTokenOut(val *MessageSubscriberAuthTokenOut) *NullableMessageSubscriberAuthTokenOut {
	return &NullableMessageSubscriberAuthTokenOut{value: val, isSet: true}
}

func (v NullableMessageSubscriberAuthTokenOut) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableMessageSubscriberAuthTokenOut) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


