// Package svix this file is @generated DO NOT EDIT
package models

type BackgroundTaskFinishedEvent2 struct {
	Data   map[string]any       `json:"data"`
	Status BackgroundTaskStatus `json:"status"`
	Task   BackgroundTaskType   `json:"task"`
	TaskId string               `json:"taskId"` // The QueueBackgroundTask's ID.
}
