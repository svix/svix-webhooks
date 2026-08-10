// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type GoogleCloudPubSubConfigPatch struct {
	ProjectId   *string `json:"projectId,omitempty"`
	TopicId     *string `json:"topicId,omitempty"`
	Credentials *string `json:"credentials,omitempty"`
}

func (o GoogleCloudPubSubConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.ProjectId != nil {
		toSerialize["projectId"] = o.ProjectId
	}
	if o.TopicId != nil {
		toSerialize["topicId"] = o.TopicId
	}
	if o.Credentials != nil {
		toSerialize["credentials"] = o.Credentials
	}
	return json.Marshal(toSerialize)
}
