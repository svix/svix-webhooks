// Package svix this file is @generated DO NOT EDIT
package models

import "encoding/json"

type BigQueryConfigPatch struct {
	ProjectId   *string `json:"projectId,omitempty"`
	DatasetId   *string `json:"datasetId,omitempty"`
	TableId     *string `json:"tableId,omitempty"`
	Credentials *string `json:"credentials,omitempty"`
}

func (o BigQueryConfigPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.ProjectId != nil {
		toSerialize["projectId"] = o.ProjectId
	}
	if o.DatasetId != nil {
		toSerialize["datasetId"] = o.DatasetId
	}
	if o.TableId != nil {
		toSerialize["tableId"] = o.TableId
	}
	if o.Credentials != nil {
		toSerialize["credentials"] = o.Credentials
	}
	return json.Marshal(toSerialize)
}
