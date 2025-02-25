// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.BackgroundTaskOut;
import com.svix.models.ListResponseBackgroundTaskOut;

import okhttp3.HttpUrl;

import java.io.IOException;

public class BackgroundTask {
    private final SvixHttpClient client;

    public BackgroundTask(SvixHttpClient client) {
        this.client = client;
    }

    /** List background tasks executed in the past 90 days. */
    public ListResponseBackgroundTaskOut list() throws IOException, ApiException {

        return this.list(new BackgroundTaskListOptions());
    }

    /** List background tasks executed in the past 90 days. */
    public ListResponseBackgroundTaskOut list(final BackgroundTaskListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/background-task");
        if (options.status != null) {
            url.addQueryParameter("status", Utils.serializeQueryParam(options.status));
        }
        if (options.task != null) {
            url.addQueryParameter("task", Utils.serializeQueryParam(options.task));
        }
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        if (options.order != null) {
            url.addQueryParameter("order", Utils.serializeQueryParam(options.order));
        }
        return this.client.executeRequest(
                "GET", url.build(), null, null, ListResponseBackgroundTaskOut.class);
    }

    /** Get a background task by ID. */
    public BackgroundTaskOut get(final String taskId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/background-task/%s", taskId));
        return this.client.executeRequest("GET", url.build(), null, null, BackgroundTaskOut.class);
    }
}
