// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.CreateStreamEventsIn;
import com.svix.models.CreateStreamEventsOut;
import com.svix.models.EventStreamOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class StreamEvents {
    private final SvixHttpClient client;

    public StreamEvents(SvixHttpClient client) {
        this.client = client;
    }

    /** Creates events on the Stream. */
    public CreateStreamEventsOut create(
            final String streamId, final CreateStreamEventsIn createStreamEventsIn)
            throws IOException, ApiException {
        return this.create(streamId, createStreamEventsIn, new StreamEventsCreateOptions());
    }

    /** Creates events on the Stream. */
    public CreateStreamEventsOut create(
            final String streamId,
            final CreateStreamEventsIn createStreamEventsIn,
            final StreamEventsCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/%s/events", streamId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                createStreamEventsIn,
                CreateStreamEventsOut.class);
    }

    /**
     * Iterate over a stream of events.
     *
     * <p>The sink must be of type `poller` to use the poller endpoint.
     */
    public EventStreamOut get(final String streamId, final String sinkId)
            throws IOException, ApiException {
        return this.get(streamId, sinkId, new StreamEventsGetOptions());
    }

    /**
     * Iterate over a stream of events.
     *
     * <p>The sink must be of type `poller` to use the poller endpoint.
     */
    public EventStreamOut get(
            final String streamId, final String sinkId, final StreamEventsGetOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/stream/%s/sink/%s/events", streamId, sinkId));
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        if (options.after != null) {
            url.addQueryParameter("after", Utils.serializeQueryParam(options.after));
        }
        return this.client.executeRequest("GET", url.build(), null, null, EventStreamOut.class);
    }
}
