// this file is @generated (with minor manual changes)
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type BackgroundTask struct {
	api *openapi.APIClient
}

type BackgroundTaskListOptions struct {
	// Filter the response based on the status.
	Status *BackgroundTaskStatus
	// Filter the response based on the type.
	Task *BackgroundTaskType
	// Limit the number of returned items
	Limit *int32
	// The iterator returned from a prior invocation
	Iterator *string
	// The sorting order of the returned items
	Order *Ordering
}

// List background tasks executed in the past 90 days.
func (backgroundTask *BackgroundTask) List(
	ctx context.Context,
	options *BackgroundTaskListOptions,
) (*ListResponseBackgroundTaskOut, error) {
	req := backgroundTask.api.BackgroundTasksAPI.ListBackgroundTasks(
		ctx,
	)

	if options != nil {
		if options.Status != nil {
			req = req.Status(*options.Status)
		}
		if options.Task != nil {
			req = req.Task(*options.Task)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Order != nil {
			req = req.Order(*options.Order)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}

// Get a background task by ID.
func (backgroundTask *BackgroundTask) Get(
	ctx context.Context,
	taskId string,
) (*BackgroundTaskOut, error) {
	req := backgroundTask.api.BackgroundTasksAPI.GetBackgroundTask(
		ctx,
		taskId,
	)

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}

	return ret, nil
}
