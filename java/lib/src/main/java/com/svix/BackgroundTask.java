package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.BackgroundTasksApi;
import com.svix.models.BackgroundTaskOut;
import com.svix.models.ListResponseBackgroundTaskOut;

public final class BackgroundTask {
    private final BackgroundTasksApi api;

    BackgroundTask() {
        api = new BackgroundTasksApi();
    }

    public ListResponseBackgroundTaskOut list(final BackgroundTaskListOptions options) throws ApiException {
        try {
            return api.listBackgroundTasks(options.getStatus(), options.getTask(), options.getLimit(),
                    options.getIterator(),
                    options.getOrder());
        } catch (com.svix.internal.ApiException e) {
            throw Utils.wrapInternalApiException(e);
        }
    }

    public BackgroundTaskOut get(final String taskId) throws ApiException {
        try {
            return api.getBackgroundTask(taskId);
        } catch (com.svix.internal.ApiException e) {
            throw Utils.wrapInternalApiException(e);
        }
    }
}
