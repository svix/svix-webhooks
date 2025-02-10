// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type MessageAttemptOut struct {
	// The ep's ID
	EndpointId string `json:"endpointId"`
	// The attempt's ID
	Id  string      `json:"id"`
	Msg *MessageOut `json:"msg,omitempty"`
	// The msg's ID
	MsgId    string `json:"msgId"`
	Response string `json:"response"`
	// Response duration in milliseconds.
	ResponseDurationMs int64                     `json:"responseDurationMs"`
	ResponseStatusCode int16                     `json:"responseStatusCode"`
	Status             MessageStatus             `json:"status"`
	Timestamp          time.Time                 `json:"timestamp"`
	TriggerType        MessageAttemptTriggerType `json:"triggerType"`
	Url                string                    `json:"url"`
}
