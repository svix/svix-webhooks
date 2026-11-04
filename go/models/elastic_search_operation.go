// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

// Operation for bulk indexing.
//
// `index` will do an upsert for the given document ID, and `create` will insert it
// (and do nothing on conflict). Only `create` is supported for Data Stream targets.
type ElasticSearchOperation string

const (
	ELASTICSEARCHOPERATION_INDEX  ElasticSearchOperation = "index"
	ELASTICSEARCHOPERATION_CREATE ElasticSearchOperation = "create"
)

var allowedElasticSearchOperation = []ElasticSearchOperation{
	"index",
	"create",
}

func (v *ElasticSearchOperation) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := ElasticSearchOperation(value)
	if slices.Contains(allowedElasticSearchOperation, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid ElasticSearchOperation", value)

}

var ElasticSearchOperationFromString = map[string]ElasticSearchOperation{
	"index":  ELASTICSEARCHOPERATION_INDEX,
	"create": ELASTICSEARCHOPERATION_CREATE,
}
