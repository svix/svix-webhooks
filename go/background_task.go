// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/models"
)

type BackgroundTask struct {
	client *SvixHttpClient
}

type BackgroundTaskListOptions struct {

	// Filter the response based on the status.
	Status *models.BackgroundTaskStatus

	// Filter the response based on the type.
	Task *models.BackgroundTaskType
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

// List background tasks executed in the past 90 days.
func (backgroundTask *BackgroundTask) List(
	ctx context.Context,
	o *BackgroundTaskListOptions,
) (*models.ListResponseBackgroundTaskOut, error) {
	pathMap := map[string]string{}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	var err error
	if o != nil {
		SerializeParamToMap("status", o.Status, queryMap, &err)
		SerializeParamToMap("task", o.Task, queryMap, &err)
		SerializeParamToMap("limit", o.Limit, queryMap, &err)
		SerializeParamToMap("iterator", o.Iterator, queryMap, &err)
		SerializeParamToMap("order", o.Order, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}
	ret, err := executeRequest[models.ListResponseBackgroundTaskOut](
		ctx,
		backgroundTask.client,
		"GET",
		"/api/v1/background-task",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}

// Get a background task by ID.
func (backgroundTask *BackgroundTask) Get(
	ctx context.Context,
	taskId string,
) (*models.BackgroundTaskOut, error) {
	pathMap := map[string]string{
		"task_id": taskId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var jsonBody []byte

	ret, err := executeRequest[models.BackgroundTaskOut](
		ctx,
		backgroundTask.client,
		"GET",
		"/api/v1/background-task/{task_id}",
		pathMap,
		queryMap,
		headerMap,
		jsonBody,
	)
	if err != nil {
		return nil, err
	}
	return ret, nil
}
