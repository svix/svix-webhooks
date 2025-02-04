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

// checks if the IntegrationIn type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &IntegrationIn{}

// IntegrationIn struct for IntegrationIn
type IntegrationIn struct {
	// The set of feature flags the integration will have access to.
	FeatureFlags []string `json:"featureFlags,omitempty"`
	Name string `json:"name"`
}

type _IntegrationIn IntegrationIn

// NewIntegrationIn instantiates a new IntegrationIn object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewIntegrationIn(name string) *IntegrationIn {
	this := IntegrationIn{}
	this.Name = name
	return &this
}

// NewIntegrationInWithDefaults instantiates a new IntegrationIn object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewIntegrationInWithDefaults() *IntegrationIn {
	this := IntegrationIn{}
	return &this
}

// GetFeatureFlags returns the FeatureFlags field value if set, zero value otherwise.
func (o *IntegrationIn) GetFeatureFlags() []string {
	if o == nil || IsNil(o.FeatureFlags) {
		var ret []string
		return ret
	}
	return o.FeatureFlags
}

// GetFeatureFlagsOk returns a tuple with the FeatureFlags field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *IntegrationIn) GetFeatureFlagsOk() ([]string, bool) {
	if o == nil || IsNil(o.FeatureFlags) {
		return nil, false
	}
	return o.FeatureFlags, true
}

// HasFeatureFlags returns a boolean if a field has been set.
func (o *IntegrationIn) HasFeatureFlags() bool {
	if o != nil && !IsNil(o.FeatureFlags) {
		return true
	}

	return false
}

// SetFeatureFlags gets a reference to the given []string and assigns it to the FeatureFlags field.
func (o *IntegrationIn) SetFeatureFlags(v []string) {
	o.FeatureFlags = v
}

// GetName returns the Name field value
func (o *IntegrationIn) GetName() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Name
}

// GetNameOk returns a tuple with the Name field value
// and a boolean to check if the value has been set.
func (o *IntegrationIn) GetNameOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Name, true
}

// SetName sets field value
func (o *IntegrationIn) SetName(v string) {
	o.Name = v
}

func (o IntegrationIn) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o IntegrationIn) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if !IsNil(o.FeatureFlags) {
		toSerialize["featureFlags"] = o.FeatureFlags
	}
	toSerialize["name"] = o.Name
	return toSerialize, nil
}

func (o *IntegrationIn) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"name",
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

	varIntegrationIn := _IntegrationIn{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varIntegrationIn)

	if err != nil {
		return err
	}

	*o = IntegrationIn(varIntegrationIn)

	return err
}

type NullableIntegrationIn struct {
	value *IntegrationIn
	isSet bool
}

func (v NullableIntegrationIn) Get() *IntegrationIn {
	return v.value
}

func (v *NullableIntegrationIn) Set(val *IntegrationIn) {
	v.value = val
	v.isSet = true
}

func (v NullableIntegrationIn) IsSet() bool {
	return v.isSet
}

func (v *NullableIntegrationIn) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableIntegrationIn(val *IntegrationIn) *NullableIntegrationIn {
	return &NullableIntegrationIn{value: val, isSet: true}
}

func (v NullableIntegrationIn) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableIntegrationIn) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


