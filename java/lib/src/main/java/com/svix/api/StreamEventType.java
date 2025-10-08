// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.ListResponseStreamEventTypeOut;
import com.svix.models.StreamEventTypeIn;
import com.svix.models.StreamEventTypeOut;
import com.svix.models.StreamEventTypePatch;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class StreamEventType {
    private final SvixHttpClient client;

    public StreamEventType(SvixHttpClient client) {
        this.client = client;
    }

    /** List of all the organization's event types for streaming. */
    public ListResponseStreamEventTypeOut list() throws IOException, ApiException {

        return this.list(new StreamEventTypeListOptions());
    }

    /** List of all the organization's event types for streaming. */
    public ListResponseStreamEventTypeOut list(final StreamEventTypeListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/stream/event-type");
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        if (options.order != null) {
            url.addQueryParameter("order", Utils.serializeQueryParam(options.order));
        }
        if (options.includeArchived != null) {
            url.addQueryParameter(
                    "include_archived", Utils.serializeQueryParam(options.includeArchived));
        }
        return this.client.executeRequest(
                "GET", url.build(), null, null, ListResponseStreamEventTypeOut.class);
    }

    /** Create an event type for Streams. */
    public StreamEventTypeOut create(final StreamEventTypeIn streamEventTypeIn)
            throws IOException, ApiException {
        return this.create(streamEventTypeIn, new StreamEventTypeCreateOptions());
    }

    /** Create an event type for Streams. */
    public StreamEventTypeOut create(
            final StreamEventTypeIn streamEventTypeIn, final StreamEventTypeCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/stream/event-type");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                streamEventTypeIn,
                StreamEventTypeOut.class);
    }

    /** Get an event type. */
    public StreamEventTypeOut get(final String name) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/event-type/%s", name));
        return this.client.executeRequest("GET", url.build(), null, null, StreamEventTypeOut.class);
    }

    /** Update or create a event type for Streams. */
    public StreamEventTypeOut update(final String name, final StreamEventTypeIn streamEventTypeIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/event-type/%s", name));
        return this.client.executeRequest(
                "PUT", url.build(), null, streamEventTypeIn, StreamEventTypeOut.class);
    }

    /** Delete an event type. */
    public void delete(final String name) throws IOException, ApiException {
        this.delete(name, new StreamEventTypeDeleteOptions());
    }

    /** Delete an event type. */
    public void delete(final String name, final StreamEventTypeDeleteOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/event-type/%s", name));
        if (options.expunge != null) {
            url.addQueryParameter("expunge", Utils.serializeQueryParam(options.expunge));
        }
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /** Patch an event type for Streams. */
    public StreamEventTypeOut patch(
            final String name, final StreamEventTypePatch streamEventTypePatch)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/event-type/%s", name));
        return this.client.executeRequest(
                "PATCH", url.build(), null, streamEventTypePatch, StreamEventTypeOut.class);
    }
}
