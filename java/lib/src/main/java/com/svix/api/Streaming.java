// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.EndpointHeadersOut;
import com.svix.models.HttpSinkHeadersPatchIn;
import com.svix.models.SinkTransformationOut;

import lombok.Getter;

import okhttp3.HttpUrl;

import java.io.IOException;

public class Streaming {
    private final SvixHttpClient client;

    @Getter private final StreamingEventType eventType;
    @Getter private final StreamingEvents events;
    @Getter private final StreamingSink sink;
    @Getter private final StreamingStream stream;

    public Streaming(SvixHttpClient client) {
        this.client = client;
        this.eventType = new StreamingEventType(client);
        this.events = new StreamingEvents(client);
        this.sink = new StreamingSink(client);
        this.stream = new StreamingStream(client);
    }

    /** Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks. */
    public EndpointHeadersOut sinkHeadersGet(final String streamId, final String sinkId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/stream/%s/sink/%s/headers", streamId, sinkId));
        return this.client.executeRequest("GET", url.build(), null, null, EndpointHeadersOut.class);
    }

    /** Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks. */
    public EndpointHeadersOut sinkHeadersPatch(
            final String streamId,
            final String sinkId,
            final HttpSinkHeadersPatchIn httpSinkHeadersPatchIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/stream/%s/sink/%s/headers", streamId, sinkId));
        return this.client.executeRequest(
                "PATCH", url.build(), null, httpSinkHeadersPatchIn, EndpointHeadersOut.class);
    }

    /** Get the transformation code associated with this sink. */
    public SinkTransformationOut sinkTransformationGet(final String streamId, final String sinkId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/stream/%s/sink/%s/transformation",
                                        streamId, sinkId));
        return this.client.executeRequest(
                "GET", url.build(), null, null, SinkTransformationOut.class);
    }
}
