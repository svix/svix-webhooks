// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ExpungeAllContentsOut struct {
	Id        string               `json:"id"` // The QueueBackgroundTask's ID.
	Status    BackgroundTaskStatus `json:"status"`
	Task      BackgroundTaskType   `json:"task"`
	UpdatedAt time.Time            `json:"updatedAt"`
}
