package models

import "encoding/json"

type Nullable[T any] struct {
	val   T
	isSet bool
}

func NewNullable[T any](value T) Nullable[T] {
	return Nullable[T]{val: value, isSet: true}
}

func (n *Nullable[T]) UnmarshalJSON(data []byte) error {
	n.isSet = true
	if string(data) == "null" {
		return nil
	}

	var value T
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}

	n.val = value
	return nil
}

func (n Nullable[T]) MarshalJSON() ([]byte, error) {
	if n.isSet {
		return json.Marshal(n.val)
	}
	return nil, nil

}
