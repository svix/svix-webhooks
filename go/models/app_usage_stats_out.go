// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type AppUsageStatsOut struct {
	// Any app IDs or UIDs received in the request that weren't found.
	//
	// Stats will be produced for all the others.
	UnresolvedAppIds []string             `json:"unresolvedAppIds"`
	Id               string               `json:"id"` // The QueueBackgroundTask's ID.
	Status           BackgroundTaskStatus `json:"status"`
	Task             BackgroundTaskType   `json:"task"`
	UpdatedAt        time.Time            `json:"updatedAt"`
}
