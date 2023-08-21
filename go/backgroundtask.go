package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type (
	ListResponseBackgroundTaskOut = openapi.ListResponseBackgroundTaskOut
	BackgroundTaskOut             = openapi.BackgroundTaskOut
)

type BackgroundTask struct {
	api *openapi.APIClient
}

type BackgroundTaskListOptions struct {
	Iterator *string
	Limit    *int32
	Order    *Ordering
	Status   *openapi.BackgroundTaskStatus
	Task     *openapi.BackgroundTaskType
}

func (a *BackgroundTask) List(ctx context.Context, options *BackgroundTaskListOptions) (*ListResponseBackgroundTaskOut, error) {
	req := a.api.BackgroundTasksApi.ListBackgroundTasks(ctx)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Order != nil {
			req = req.Order(openapi.Ordering(*options.Order))
		}
		if options.Status != nil {
			req = req.Status(*options.Status)
		}
		if options.Task != nil {
			req = req.Task(*options.Task)
		}
	}
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseBackgroundTaskOut(resp)
	return &ret, nil
}

func (a *BackgroundTask) Get(ctx context.Context, taskId string) (*BackgroundTaskOut, error) {
	req := a.api.BackgroundTasksApi.GetBackgroundTask(ctx, taskId)
	resp, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := BackgroundTaskOut(resp)
	return &ret, nil
}
