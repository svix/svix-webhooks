// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type MessageStatusText string

const (
	MESSAGESTATUSTEXT_SUCCESS  MessageStatusText = "success"
	MESSAGESTATUSTEXT_PENDING  MessageStatusText = "pending"
	MESSAGESTATUSTEXT_FAIL     MessageStatusText = "fail"
	MESSAGESTATUSTEXT_SENDING  MessageStatusText = "sending"
	MESSAGESTATUSTEXT_CANCELED MessageStatusText = "canceled"
)

var allowedMessageStatusText = []MessageStatusText{
	"success",
	"pending",
	"fail",
	"sending",
	"canceled",
}

func (v *MessageStatusText) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := MessageStatusText(value)
	if slices.Contains(allowedMessageStatusText, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid MessageStatusText", value)

}

var MessageStatusTextFromString = map[string]MessageStatusText{
	"success":  MESSAGESTATUSTEXT_SUCCESS,
	"pending":  MESSAGESTATUSTEXT_PENDING,
	"fail":     MESSAGESTATUSTEXT_FAIL,
	"sending":  MESSAGESTATUSTEXT_SENDING,
	"canceled": MESSAGESTATUSTEXT_CANCELED,
}
