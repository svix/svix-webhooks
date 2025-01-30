/*
Svix API

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

API version: 1.1.1
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
)

// checks if the StreamPatch type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &StreamPatch{}

// StreamPatch struct for StreamPatch
type StreamPatch struct {
	// The Stream's description.
	Description *string `json:"description,omitempty"`
	// The Stream's UID.
	Uid *string `json:"uid,omitempty" validate:"regexp=^(?!strm_)[a-zA-Z0-9_-]+$"`
}

// NewStreamPatch instantiates a new StreamPatch object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewStreamPatch() *StreamPatch {
	this := StreamPatch{}
	return &this
}

// NewStreamPatchWithDefaults instantiates a new StreamPatch object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewStreamPatchWithDefaults() *StreamPatch {
	this := StreamPatch{}
	return &this
}

// GetDescription returns the Description field value if set, zero value otherwise.
func (o *StreamPatch) GetDescription() string {
	if o == nil || IsNil(o.Description) {
		var ret string
		return ret
	}
	return *o.Description
}

// GetDescriptionOk returns a tuple with the Description field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *StreamPatch) GetDescriptionOk() (*string, bool) {
	if o == nil || IsNil(o.Description) {
		return nil, false
	}
	return o.Description, true
}

// HasDescription returns a boolean if a field has been set.
func (o *StreamPatch) HasDescription() bool {
	if o != nil && !IsNil(o.Description) {
		return true
	}

	return false
}

// SetDescription gets a reference to the given string and assigns it to the Description field.
func (o *StreamPatch) SetDescription(v string) {
	o.Description = &v
}

// GetUid returns the Uid field value if set, zero value otherwise.
func (o *StreamPatch) GetUid() string {
	if o == nil || IsNil(o.Uid) {
		var ret string
		return ret
	}
	return *o.Uid
}

// GetUidOk returns a tuple with the Uid field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *StreamPatch) GetUidOk() (*string, bool) {
	if o == nil || IsNil(o.Uid) {
		return nil, false
	}
	return o.Uid, true
}

// HasUid returns a boolean if a field has been set.
func (o *StreamPatch) HasUid() bool {
	if o != nil && !IsNil(o.Uid) {
		return true
	}

	return false
}

// SetUid gets a reference to the given string and assigns it to the Uid field.
func (o *StreamPatch) SetUid(v string) {
	o.Uid = &v
}

func (o StreamPatch) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o StreamPatch) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if !IsNil(o.Description) {
		toSerialize["description"] = o.Description
	}
	if !IsNil(o.Uid) {
		toSerialize["uid"] = o.Uid
	}
	return toSerialize, nil
}

type NullableStreamPatch struct {
	value *StreamPatch
	isSet bool
}

func (v NullableStreamPatch) Get() *StreamPatch {
	return v.value
}

func (v *NullableStreamPatch) Set(val *StreamPatch) {
	v.value = val
	v.isSet = true
}

func (v NullableStreamPatch) IsSet() bool {
	return v.isSet
}

func (v *NullableStreamPatch) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableStreamPatch(val *StreamPatch) *NullableStreamPatch {
	return &NullableStreamPatch{value: val, isSet: true}
}

func (v NullableStreamPatch) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableStreamPatch) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


