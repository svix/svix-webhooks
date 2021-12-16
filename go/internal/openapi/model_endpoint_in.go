/*
 * Svix API
 *
 * Welcome to the Svix API documentation!  Useful links: [Homepage](https://www.svix.com) | [Support email](mailto:support+docs@svix.com) | [Blog](https://www.svix.com/blog/) | [Slack Community](https://www.svix.com/slack/)  # Introduction  This is the reference documentation and schemas for the [Svix webhook service](https://www.svix.com) API. For tutorials and other documentation please refer to [the documentation](https://docs.svix.com).  ## Main concepts  In Svix you have four important entities you will be interacting with:  - `messages`: these are the webhooks being sent. They can have contents and a few other properties. - `application`: this is where `messages` are sent to. Usually you want to create one application for each user on your platform. - `endpoint`: endpoints are the URLs messages will be sent to. Each application can have multiple `endpoints` and each message sent to that application will be sent to all of them (unless they are not subscribed to the sent event type). - `event-type`: event types are identifiers denoting the type of the message being sent. Event types are primarily used to decide which events are sent to which endpoint.   ## Authentication  Get your authentication token (`AUTH_TOKEN`) from the [Svix dashboard](https://dashboard.svix.com) and use it as part of the `Authorization` header as such: `Authorization: Bearer ${AUTH_TOKEN}`.  <SecurityDefinitions />   ## Code samples  The code samples assume you already have the respective libraries installed and you know how to use them. For the latest information on how to do that, please refer to [the documentation](https://docs.svix.com/).   ## Cross-Origin Resource Sharing  This API features Cross-Origin Resource Sharing (CORS) implemented in compliance with [W3C spec](https://www.w3.org/TR/cors/). And that allows cross-domain communication from the browser. All responses have a wildcard same-origin which makes them completely public and accessible to everyone, including any code on any site. 
 *
 * API version: 1.4
 */

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
)

// EndpointIn struct for EndpointIn
type EndpointIn struct {
	Description *string `json:"description,omitempty"`
	Disabled *bool `json:"disabled,omitempty"`
	FilterTypes *[]string `json:"filterTypes,omitempty"`
	RateLimit *int32 `json:"rateLimit,omitempty"`
	// The endpoint's verification secret. If `null` is passed, a secret is automatically generated.
	Secret *string `json:"secret,omitempty"`
	// Optional unique identifier for the endpoint
	Uid *string `json:"uid,omitempty"`
	Url string `json:"url"`
	Version int32 `json:"version"`
}

// NewEndpointIn instantiates a new EndpointIn object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewEndpointIn(url string, version int32) *EndpointIn {
	this := EndpointIn{}
	var description string = ""
	this.Description = &description
	var disabled bool = false
	this.Disabled = &disabled
	this.Url = url
	this.Version = version
	return &this
}

// NewEndpointInWithDefaults instantiates a new EndpointIn object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewEndpointInWithDefaults() *EndpointIn {
	this := EndpointIn{}
	var description string = ""
	this.Description = &description
	var disabled bool = false
	this.Disabled = &disabled
	return &this
}

// GetDescription returns the Description field value if set, zero value otherwise.
func (o *EndpointIn) GetDescription() string {
	if o == nil || o.Description == nil {
		var ret string
		return ret
	}
	return *o.Description
}

// GetDescriptionOk returns a tuple with the Description field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *EndpointIn) GetDescriptionOk() (*string, bool) {
	if o == nil || o.Description == nil {
		return nil, false
	}
	return o.Description, true
}

// HasDescription returns a boolean if a field has been set.
func (o *EndpointIn) HasDescription() bool {
	if o != nil && o.Description != nil {
		return true
	}

	return false
}

// SetDescription gets a reference to the given string and assigns it to the Description field.
func (o *EndpointIn) SetDescription(v string) {
	o.Description = &v
}

// GetDisabled returns the Disabled field value if set, zero value otherwise.
func (o *EndpointIn) GetDisabled() bool {
	if o == nil || o.Disabled == nil {
		var ret bool
		return ret
	}
	return *o.Disabled
}

// GetDisabledOk returns a tuple with the Disabled field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *EndpointIn) GetDisabledOk() (*bool, bool) {
	if o == nil || o.Disabled == nil {
		return nil, false
	}
	return o.Disabled, true
}

// HasDisabled returns a boolean if a field has been set.
func (o *EndpointIn) HasDisabled() bool {
	if o != nil && o.Disabled != nil {
		return true
	}

	return false
}

// SetDisabled gets a reference to the given bool and assigns it to the Disabled field.
func (o *EndpointIn) SetDisabled(v bool) {
	o.Disabled = &v
}

// GetFilterTypes returns the FilterTypes field value if set, zero value otherwise.
func (o *EndpointIn) GetFilterTypes() []string {
	if o == nil || o.FilterTypes == nil {
		var ret []string
		return ret
	}
	return *o.FilterTypes
}

// GetFilterTypesOk returns a tuple with the FilterTypes field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *EndpointIn) GetFilterTypesOk() (*[]string, bool) {
	if o == nil || o.FilterTypes == nil {
		return nil, false
	}
	return o.FilterTypes, true
}

// HasFilterTypes returns a boolean if a field has been set.
func (o *EndpointIn) HasFilterTypes() bool {
	if o != nil && o.FilterTypes != nil {
		return true
	}

	return false
}

// SetFilterTypes gets a reference to the given []string and assigns it to the FilterTypes field.
func (o *EndpointIn) SetFilterTypes(v []string) {
	o.FilterTypes = &v
}

// GetRateLimit returns the RateLimit field value if set, zero value otherwise.
func (o *EndpointIn) GetRateLimit() int32 {
	if o == nil || o.RateLimit == nil {
		var ret int32
		return ret
	}
	return *o.RateLimit
}

// GetRateLimitOk returns a tuple with the RateLimit field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *EndpointIn) GetRateLimitOk() (*int32, bool) {
	if o == nil || o.RateLimit == nil {
		return nil, false
	}
	return o.RateLimit, true
}

// HasRateLimit returns a boolean if a field has been set.
func (o *EndpointIn) HasRateLimit() bool {
	if o != nil && o.RateLimit != nil {
		return true
	}

	return false
}

// SetRateLimit gets a reference to the given int32 and assigns it to the RateLimit field.
func (o *EndpointIn) SetRateLimit(v int32) {
	o.RateLimit = &v
}

// GetSecret returns the Secret field value if set, zero value otherwise.
func (o *EndpointIn) GetSecret() string {
	if o == nil || o.Secret == nil {
		var ret string
		return ret
	}
	return *o.Secret
}

// GetSecretOk returns a tuple with the Secret field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *EndpointIn) GetSecretOk() (*string, bool) {
	if o == nil || o.Secret == nil {
		return nil, false
	}
	return o.Secret, true
}

// HasSecret returns a boolean if a field has been set.
func (o *EndpointIn) HasSecret() bool {
	if o != nil && o.Secret != nil {
		return true
	}

	return false
}

// SetSecret gets a reference to the given string and assigns it to the Secret field.
func (o *EndpointIn) SetSecret(v string) {
	o.Secret = &v
}

// GetUid returns the Uid field value if set, zero value otherwise.
func (o *EndpointIn) GetUid() string {
	if o == nil || o.Uid == nil {
		var ret string
		return ret
	}
	return *o.Uid
}

// GetUidOk returns a tuple with the Uid field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *EndpointIn) GetUidOk() (*string, bool) {
	if o == nil || o.Uid == nil {
		return nil, false
	}
	return o.Uid, true
}

// HasUid returns a boolean if a field has been set.
func (o *EndpointIn) HasUid() bool {
	if o != nil && o.Uid != nil {
		return true
	}

	return false
}

// SetUid gets a reference to the given string and assigns it to the Uid field.
func (o *EndpointIn) SetUid(v string) {
	o.Uid = &v
}

// GetUrl returns the Url field value
func (o *EndpointIn) GetUrl() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Url
}

// GetUrlOk returns a tuple with the Url field value
// and a boolean to check if the value has been set.
func (o *EndpointIn) GetUrlOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.Url, true
}

// SetUrl sets field value
func (o *EndpointIn) SetUrl(v string) {
	o.Url = v
}

// GetVersion returns the Version field value
func (o *EndpointIn) GetVersion() int32 {
	if o == nil {
		var ret int32
		return ret
	}

	return o.Version
}

// GetVersionOk returns a tuple with the Version field value
// and a boolean to check if the value has been set.
func (o *EndpointIn) GetVersionOk() (*int32, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.Version, true
}

// SetVersion sets field value
func (o *EndpointIn) SetVersion(v int32) {
	o.Version = v
}

func (o EndpointIn) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Description != nil {
		toSerialize["description"] = o.Description
	}
	if o.Disabled != nil {
		toSerialize["disabled"] = o.Disabled
	}
	if o.FilterTypes != nil {
		toSerialize["filterTypes"] = o.FilterTypes
	}
	if o.RateLimit != nil {
		toSerialize["rateLimit"] = o.RateLimit
	}
	if o.Secret != nil {
		toSerialize["secret"] = o.Secret
	}
	if o.Uid != nil {
		toSerialize["uid"] = o.Uid
	}
	if true {
		toSerialize["url"] = o.Url
	}
	if true {
		toSerialize["version"] = o.Version
	}
	return json.Marshal(toSerialize)
}

type NullableEndpointIn struct {
	value *EndpointIn
	isSet bool
}

func (v NullableEndpointIn) Get() *EndpointIn {
	return v.value
}

func (v *NullableEndpointIn) Set(val *EndpointIn) {
	v.value = val
	v.isSet = true
}

func (v NullableEndpointIn) IsSet() bool {
	return v.isSet
}

func (v *NullableEndpointIn) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableEndpointIn(val *EndpointIn) *NullableEndpointIn {
	return &NullableEndpointIn{value: val, isSet: true}
}

func (v NullableEndpointIn) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableEndpointIn) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


