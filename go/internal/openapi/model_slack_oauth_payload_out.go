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

// SlackOauthPayloadOut struct for SlackOauthPayloadOut
type SlackOauthPayloadOut struct {
	Channel NullableString `json:"channel,omitempty"`
	Error NullableString `json:"error,omitempty"`
	IncomingWebhookUrl NullableString `json:"incomingWebhookUrl,omitempty"`
	Ok bool `json:"ok"`
}

// NewSlackOauthPayloadOut instantiates a new SlackOauthPayloadOut object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewSlackOauthPayloadOut(ok bool) *SlackOauthPayloadOut {
	this := SlackOauthPayloadOut{}
	this.Ok = ok
	return &this
}

// NewSlackOauthPayloadOutWithDefaults instantiates a new SlackOauthPayloadOut object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewSlackOauthPayloadOutWithDefaults() *SlackOauthPayloadOut {
	this := SlackOauthPayloadOut{}
	return &this
}

// GetChannel returns the Channel field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *SlackOauthPayloadOut) GetChannel() string {
	if o == nil || o.Channel.Get() == nil {
		var ret string
		return ret
	}
	return *o.Channel.Get()
}

// GetChannelOk returns a tuple with the Channel field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *SlackOauthPayloadOut) GetChannelOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return o.Channel.Get(), o.Channel.IsSet()
}

// HasChannel returns a boolean if a field has been set.
func (o *SlackOauthPayloadOut) HasChannel() bool {
	if o != nil && o.Channel.IsSet() {
		return true
	}

	return false
}

// SetChannel gets a reference to the given NullableString and assigns it to the Channel field.
func (o *SlackOauthPayloadOut) SetChannel(v string) {
	o.Channel.Set(&v)
}
// SetChannelNil sets the value for Channel to be an explicit nil
func (o *SlackOauthPayloadOut) SetChannelNil() {
	o.Channel.Set(nil)
}

// UnsetChannel ensures that no value is present for Channel, not even an explicit nil
func (o *SlackOauthPayloadOut) UnsetChannel() {
	o.Channel.Unset()
}

// GetError returns the Error field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *SlackOauthPayloadOut) GetError() string {
	if o == nil || o.Error.Get() == nil {
		var ret string
		return ret
	}
	return *o.Error.Get()
}

// GetErrorOk returns a tuple with the Error field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *SlackOauthPayloadOut) GetErrorOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return o.Error.Get(), o.Error.IsSet()
}

// HasError returns a boolean if a field has been set.
func (o *SlackOauthPayloadOut) HasError() bool {
	if o != nil && o.Error.IsSet() {
		return true
	}

	return false
}

// SetError gets a reference to the given NullableString and assigns it to the Error field.
func (o *SlackOauthPayloadOut) SetError(v string) {
	o.Error.Set(&v)
}
// SetErrorNil sets the value for Error to be an explicit nil
func (o *SlackOauthPayloadOut) SetErrorNil() {
	o.Error.Set(nil)
}

// UnsetError ensures that no value is present for Error, not even an explicit nil
func (o *SlackOauthPayloadOut) UnsetError() {
	o.Error.Unset()
}

// GetIncomingWebhookUrl returns the IncomingWebhookUrl field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *SlackOauthPayloadOut) GetIncomingWebhookUrl() string {
	if o == nil || o.IncomingWebhookUrl.Get() == nil {
		var ret string
		return ret
	}
	return *o.IncomingWebhookUrl.Get()
}

// GetIncomingWebhookUrlOk returns a tuple with the IncomingWebhookUrl field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *SlackOauthPayloadOut) GetIncomingWebhookUrlOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return o.IncomingWebhookUrl.Get(), o.IncomingWebhookUrl.IsSet()
}

// HasIncomingWebhookUrl returns a boolean if a field has been set.
func (o *SlackOauthPayloadOut) HasIncomingWebhookUrl() bool {
	if o != nil && o.IncomingWebhookUrl.IsSet() {
		return true
	}

	return false
}

// SetIncomingWebhookUrl gets a reference to the given NullableString and assigns it to the IncomingWebhookUrl field.
func (o *SlackOauthPayloadOut) SetIncomingWebhookUrl(v string) {
	o.IncomingWebhookUrl.Set(&v)
}
// SetIncomingWebhookUrlNil sets the value for IncomingWebhookUrl to be an explicit nil
func (o *SlackOauthPayloadOut) SetIncomingWebhookUrlNil() {
	o.IncomingWebhookUrl.Set(nil)
}

// UnsetIncomingWebhookUrl ensures that no value is present for IncomingWebhookUrl, not even an explicit nil
func (o *SlackOauthPayloadOut) UnsetIncomingWebhookUrl() {
	o.IncomingWebhookUrl.Unset()
}

// GetOk returns the Ok field value
func (o *SlackOauthPayloadOut) GetOk() bool {
	if o == nil {
		var ret bool
		return ret
	}

	return o.Ok
}

// GetOkOk returns a tuple with the Ok field value
// and a boolean to check if the value has been set.
func (o *SlackOauthPayloadOut) GetOkOk() (*bool, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.Ok, true
}

// SetOk sets field value
func (o *SlackOauthPayloadOut) SetOk(v bool) {
	o.Ok = v
}

func (o SlackOauthPayloadOut) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Channel.IsSet() {
		toSerialize["channel"] = o.Channel.Get()
	}
	if o.Error.IsSet() {
		toSerialize["error"] = o.Error.Get()
	}
	if o.IncomingWebhookUrl.IsSet() {
		toSerialize["incomingWebhookUrl"] = o.IncomingWebhookUrl.Get()
	}
	if true {
		toSerialize["ok"] = o.Ok
	}
	return json.Marshal(toSerialize)
}

type NullableSlackOauthPayloadOut struct {
	value *SlackOauthPayloadOut
	isSet bool
}

func (v NullableSlackOauthPayloadOut) Get() *SlackOauthPayloadOut {
	return v.value
}

func (v *NullableSlackOauthPayloadOut) Set(val *SlackOauthPayloadOut) {
	v.value = val
	v.isSet = true
}

func (v NullableSlackOauthPayloadOut) IsSet() bool {
	return v.isSet
}

func (v *NullableSlackOauthPayloadOut) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableSlackOauthPayloadOut(val *SlackOauthPayloadOut) *NullableSlackOauthPayloadOut {
	return &NullableSlackOauthPayloadOut{value: val, isSet: true}
}

func (v NullableSlackOauthPayloadOut) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableSlackOauthPayloadOut) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


