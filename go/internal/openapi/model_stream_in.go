/*
Svix API

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

API version: 1.1.1
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
	"bytes"
	"fmt"
)

// checks if the StreamIn type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &StreamIn{}

// StreamIn struct for StreamIn
type StreamIn struct {
	// The stream's description.
	Description string `json:"description"`
	// The Stream's UID.
	Uid *string `json:"uid,omitempty" validate:"regexp=^(?!strm_)[a-zA-Z0-9_-]+$"`
}

type _StreamIn StreamIn

// NewStreamIn instantiates a new StreamIn object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewStreamIn(description string) *StreamIn {
	this := StreamIn{}
	this.Description = description
	return &this
}

// NewStreamInWithDefaults instantiates a new StreamIn object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewStreamInWithDefaults() *StreamIn {
	this := StreamIn{}
	return &this
}

// GetDescription returns the Description field value
func (o *StreamIn) GetDescription() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Description
}

// GetDescriptionOk returns a tuple with the Description field value
// and a boolean to check if the value has been set.
func (o *StreamIn) GetDescriptionOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Description, true
}

// SetDescription sets field value
func (o *StreamIn) SetDescription(v string) {
	o.Description = v
}

// GetUid returns the Uid field value if set, zero value otherwise.
func (o *StreamIn) GetUid() string {
	if o == nil || IsNil(o.Uid) {
		var ret string
		return ret
	}
	return *o.Uid
}

// GetUidOk returns a tuple with the Uid field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *StreamIn) GetUidOk() (*string, bool) {
	if o == nil || IsNil(o.Uid) {
		return nil, false
	}
	return o.Uid, true
}

// HasUid returns a boolean if a field has been set.
func (o *StreamIn) HasUid() bool {
	if o != nil && !IsNil(o.Uid) {
		return true
	}

	return false
}

// SetUid gets a reference to the given string and assigns it to the Uid field.
func (o *StreamIn) SetUid(v string) {
	o.Uid = &v
}

func (o StreamIn) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o StreamIn) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	toSerialize["description"] = o.Description
	if !IsNil(o.Uid) {
		toSerialize["uid"] = o.Uid
	}
	return toSerialize, nil
}

func (o *StreamIn) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"description",
	}

	allProperties := make(map[string]interface{})

	err = json.Unmarshal(data, &allProperties)

	if err != nil {
		return err;
	}

	for _, requiredProperty := range(requiredProperties) {
		if _, exists := allProperties[requiredProperty]; !exists {
			return fmt.Errorf("no value given for required property %v", requiredProperty)
		}
	}

	varStreamIn := _StreamIn{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varStreamIn)

	if err != nil {
		return err
	}

	*o = StreamIn(varStreamIn)

	return err
}

type NullableStreamIn struct {
	value *StreamIn
	isSet bool
}

func (v NullableStreamIn) Get() *StreamIn {
	return v.value
}

func (v *NullableStreamIn) Set(val *StreamIn) {
	v.value = val
	v.isSet = true
}

func (v NullableStreamIn) IsSet() bool {
	return v.isSet
}

func (v *NullableStreamIn) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableStreamIn(val *StreamIn) *NullableStreamIn {
	return &NullableStreamIn{value: val, isSet: true}
}

func (v NullableStreamIn) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableStreamIn) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


