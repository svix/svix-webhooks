// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.ListResponseStreamOut;
import com.svix.models.StreamIn;
import com.svix.models.StreamOut;
import com.svix.models.StreamPatch;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class StreamStream {
    private final SvixHttpClient client;

    public StreamStream(SvixHttpClient client) {
        this.client = client;
    }

    /** List of all the organization's streams. */
    public ListResponseStreamOut list() throws IOException, ApiException {

        return this.list(new StreamStreamListOptions());
    }

    /** List of all the organization's streams. */
    public ListResponseStreamOut list(final StreamStreamListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/stream");
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
                "GET", url.build(), null, null, ListResponseStreamOut.class);
    }

    /** Creates a new stream. */
    public StreamOut create(final StreamIn streamIn) throws IOException, ApiException {
        return this.create(streamIn, new StreamStreamCreateOptions());
    }

    /** Creates a new stream. */
    public StreamOut create(final StreamIn streamIn, final StreamStreamCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/stream");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), streamIn, StreamOut.class);
    }

    /** Get a stream by id or uid. */
    public StreamOut get(final String streamId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/%s", streamId));
        return this.client.executeRequest("GET", url.build(), null, null, StreamOut.class);
    }

    /** Update a stream. */
    public StreamOut update(final String streamId, final StreamIn streamIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/%s", streamId));
        return this.client.executeRequest("PUT", url.build(), null, streamIn, StreamOut.class);
    }

    /** Delete a stream. */
    public void delete(final String streamId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/%s", streamId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /** Partially update a stream. */
    public StreamOut patch(final String streamId, final StreamPatch streamPatch)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/%s", streamId));
        return this.client.executeRequest("PATCH", url.build(), null, streamPatch, StreamOut.class);
    }
}
