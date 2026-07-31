// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type MessageAttemptOut struct {
	Url                string                    `json:"url"`
	Response           string                    `json:"response"`
	ResponseStatusCode int16                     `json:"responseStatusCode"`
	ResponseDurationMs int64                     `json:"responseDurationMs"` // Response duration in milliseconds.
	Status             MessageStatus             `json:"status"`
	StatusText         MessageStatusText         `json:"statusText"`
	TriggerType        MessageAttemptTriggerType `json:"triggerType"`
	MsgId              string                    `json:"msgId"`      // The Message's ID.
	EndpointId         string                    `json:"endpointId"` // The Endpoint's ID.
	Id                 string                    `json:"id"`         // The MessageAttempt's ID.
	Timestamp          time.Time                 `json:"timestamp"`
	Msg                *MessageOut               `json:"msg,omitempty"`
}
