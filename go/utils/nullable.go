package utils

import "encoding/json"

type Nullable[T any] struct {
	val   *T
	isSet bool
}

// Create a new Nullable[T] that is explicitly set as null
func NewExplicitlySetNilNullable[T any]() Nullable[T] {
	return Nullable[T]{val: nil, isSet: true}
}

// Create a new Nullable[T] that is unset, This type will not be emitted when serializing
func NewUnsetNullable[T any]() Nullable[T] {
	return Nullable[T]{val: nil, isSet: false}
}

func NewNullable[T any](value T) Nullable[T] {
	return Nullable[T]{val: &value, isSet: true}
}

func (v Nullable[T]) Get() *T {
	return v.val
}

func (v *Nullable[T]) Set(val *T) {
	v.val = val
	v.isSet = true
}

func (v Nullable[T]) IsSet() bool {
	return v.isSet
}

func (v *Nullable[T]) Unset() {
	v.val = nil
	v.isSet = false
}

// Set the nullable value to nil, the when json serializing a null will be emitted
func (v *Nullable[T]) SetNil() {
	v.val = nil
	v.isSet = true
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

	n.val = &value
	return nil
}

func (n Nullable[T]) MarshalJSON() ([]byte, error) {
	if n.isSet {
		return json.Marshal(n.val)
	}
	return nil, nil

}
