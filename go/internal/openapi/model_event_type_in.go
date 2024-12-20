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

// checks if the EventTypeIn type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &EventTypeIn{}

// EventTypeIn struct for EventTypeIn
type EventTypeIn struct {
	Archived *bool `json:"archived,omitempty"`
	Deprecated *bool `json:"deprecated,omitempty"`
	Description string `json:"description"`
	FeatureFlag *string `json:"featureFlag,omitempty" validate:"regexp=^[a-zA-Z0-9\\\\-_.]+$"`
	// The event type group's name
	GroupName *string `json:"groupName,omitempty" validate:"regexp=^[a-zA-Z0-9\\\\-_.]+$"`
	// The event type's name
	Name string `json:"name" validate:"regexp=^[a-zA-Z0-9\\\\-_.]+$"`
	// The schema for the event type for a specific version as a JSON schema.
	Schemas map[string]interface{} `json:"schemas,omitempty"`
}

type _EventTypeIn EventTypeIn

// NewEventTypeIn instantiates a new EventTypeIn object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewEventTypeIn(description string, name string) *EventTypeIn {
	this := EventTypeIn{}
	var archived bool = false
	this.Archived = &archived
	var deprecated bool = false
	this.Deprecated = &deprecated
	this.Description = description
	this.Name = name
	return &this
}

// NewEventTypeInWithDefaults instantiates a new EventTypeIn object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewEventTypeInWithDefaults() *EventTypeIn {
	this := EventTypeIn{}
	var archived bool = false
	this.Archived = &archived
	var deprecated bool = false
	this.Deprecated = &deprecated
	return &this
}

// GetArchived returns the Archived field value if set, zero value otherwise.
func (o *EventTypeIn) GetArchived() bool {
	if o == nil || IsNil(o.Archived) {
		var ret bool
		return ret
	}
	return *o.Archived
}

// GetArchivedOk returns a tuple with the Archived field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *EventTypeIn) GetArchivedOk() (*bool, bool) {
	if o == nil || IsNil(o.Archived) {
		return nil, false
	}
	return o.Archived, true
}

// HasArchived returns a boolean if a field has been set.
func (o *EventTypeIn) HasArchived() bool {
	if o != nil && !IsNil(o.Archived) {
		return true
	}

	return false
}

// SetArchived gets a reference to the given bool and assigns it to the Archived field.
func (o *EventTypeIn) SetArchived(v bool) {
	o.Archived = &v
}

// GetDeprecated returns the Deprecated field value if set, zero value otherwise.
func (o *EventTypeIn) GetDeprecated() bool {
	if o == nil || IsNil(o.Deprecated) {
		var ret bool
		return ret
	}
	return *o.Deprecated
}

// GetDeprecatedOk returns a tuple with the Deprecated field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *EventTypeIn) GetDeprecatedOk() (*bool, bool) {
	if o == nil || IsNil(o.Deprecated) {
		return nil, false
	}
	return o.Deprecated, true
}

// HasDeprecated returns a boolean if a field has been set.
func (o *EventTypeIn) HasDeprecated() bool {
	if o != nil && !IsNil(o.Deprecated) {
		return true
	}

	return false
}

// SetDeprecated gets a reference to the given bool and assigns it to the Deprecated field.
func (o *EventTypeIn) SetDeprecated(v bool) {
	o.Deprecated = &v
}

// GetDescription returns the Description field value
func (o *EventTypeIn) GetDescription() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Description
}

// GetDescriptionOk returns a tuple with the Description field value
// and a boolean to check if the value has been set.
func (o *EventTypeIn) GetDescriptionOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Description, true
}

// SetDescription sets field value
func (o *EventTypeIn) SetDescription(v string) {
	o.Description = v
}

// GetFeatureFlag returns the FeatureFlag field value if set, zero value otherwise.
func (o *EventTypeIn) GetFeatureFlag() string {
	if o == nil || IsNil(o.FeatureFlag) {
		var ret string
		return ret
	}
	return *o.FeatureFlag
}

// GetFeatureFlagOk returns a tuple with the FeatureFlag field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *EventTypeIn) GetFeatureFlagOk() (*string, bool) {
	if o == nil || IsNil(o.FeatureFlag) {
		return nil, false
	}
	return o.FeatureFlag, true
}

// HasFeatureFlag returns a boolean if a field has been set.
func (o *EventTypeIn) HasFeatureFlag() bool {
	if o != nil && !IsNil(o.FeatureFlag) {
		return true
	}

	return false
}

// SetFeatureFlag gets a reference to the given string and assigns it to the FeatureFlag field.
func (o *EventTypeIn) SetFeatureFlag(v string) {
	o.FeatureFlag = &v
}

// GetGroupName returns the GroupName field value if set, zero value otherwise.
func (o *EventTypeIn) GetGroupName() string {
	if o == nil || IsNil(o.GroupName) {
		var ret string
		return ret
	}
	return *o.GroupName
}

// GetGroupNameOk returns a tuple with the GroupName field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *EventTypeIn) GetGroupNameOk() (*string, bool) {
	if o == nil || IsNil(o.GroupName) {
		return nil, false
	}
	return o.GroupName, true
}

// HasGroupName returns a boolean if a field has been set.
func (o *EventTypeIn) HasGroupName() bool {
	if o != nil && !IsNil(o.GroupName) {
		return true
	}

	return false
}

// SetGroupName gets a reference to the given string and assigns it to the GroupName field.
func (o *EventTypeIn) SetGroupName(v string) {
	o.GroupName = &v
}

// GetName returns the Name field value
func (o *EventTypeIn) GetName() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Name
}

// GetNameOk returns a tuple with the Name field value
// and a boolean to check if the value has been set.
func (o *EventTypeIn) GetNameOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Name, true
}

// SetName sets field value
func (o *EventTypeIn) SetName(v string) {
	o.Name = v
}

// GetSchemas returns the Schemas field value if set, zero value otherwise.
func (o *EventTypeIn) GetSchemas() map[string]interface{} {
	if o == nil || IsNil(o.Schemas) {
		var ret map[string]interface{}
		return ret
	}
	return o.Schemas
}

// GetSchemasOk returns a tuple with the Schemas field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *EventTypeIn) GetSchemasOk() (map[string]interface{}, bool) {
	if o == nil || IsNil(o.Schemas) {
		return map[string]interface{}{}, false
	}
	return o.Schemas, true
}

// HasSchemas returns a boolean if a field has been set.
func (o *EventTypeIn) HasSchemas() bool {
	if o != nil && !IsNil(o.Schemas) {
		return true
	}

	return false
}

// SetSchemas gets a reference to the given map[string]interface{} and assigns it to the Schemas field.
func (o *EventTypeIn) SetSchemas(v map[string]interface{}) {
	o.Schemas = v
}

func (o EventTypeIn) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o EventTypeIn) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if !IsNil(o.Archived) {
		toSerialize["archived"] = o.Archived
	}
	if !IsNil(o.Deprecated) {
		toSerialize["deprecated"] = o.Deprecated
	}
	toSerialize["description"] = o.Description
	if !IsNil(o.FeatureFlag) {
		toSerialize["featureFlag"] = o.FeatureFlag
	}
	if !IsNil(o.GroupName) {
		toSerialize["groupName"] = o.GroupName
	}
	toSerialize["name"] = o.Name
	if !IsNil(o.Schemas) {
		toSerialize["schemas"] = o.Schemas
	}
	return toSerialize, nil
}

func (o *EventTypeIn) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"description",
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

	varEventTypeIn := _EventTypeIn{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varEventTypeIn)

	if err != nil {
		return err
	}

	*o = EventTypeIn(varEventTypeIn)

	return err
}

type NullableEventTypeIn struct {
	value *EventTypeIn
	isSet bool
}

func (v NullableEventTypeIn) Get() *EventTypeIn {
	return v.value
}

func (v *NullableEventTypeIn) Set(val *EventTypeIn) {
	v.value = val
	v.isSet = true
}

func (v NullableEventTypeIn) IsSet() bool {
	return v.isSet
}

func (v *NullableEventTypeIn) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableEventTypeIn(val *EventTypeIn) *NullableEventTypeIn {
	return &NullableEventTypeIn{value: val, isSet: true}
}

func (v NullableEventTypeIn) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableEventTypeIn) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


