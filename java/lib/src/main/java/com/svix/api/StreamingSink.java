// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.EmptyResponse;
import com.svix.models.EndpointSecretRotateIn;
import com.svix.models.ListResponseStreamSinkOut;
import com.svix.models.SinkSecretOut;
import com.svix.models.SinkTransformIn;
import com.svix.models.StreamSinkIn;
import com.svix.models.StreamSinkOut;
import com.svix.models.StreamSinkPatch;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class StreamingSink {
    private final SvixHttpClient client;

    public StreamingSink(SvixHttpClient client) {
        this.client = client;
    }

    /** List of all the stream's sinks. */
    public ListResponseStreamSinkOut list(final String streamId) throws IOException, ApiException {
        return this.list(streamId, new StreamingSinkListOptions());
    }

    /** List of all the stream's sinks. */
    public ListResponseStreamSinkOut list(
            final String streamId, final StreamingSinkListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/%s/sink", streamId));
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
                "GET", url.build(), null, null, ListResponseStreamSinkOut.class);
    }

    /** Creates a new sink. */
    public StreamSinkOut create(final String streamId, final StreamSinkIn streamSinkIn)
            throws IOException, ApiException {
        return this.create(streamId, streamSinkIn, new StreamingSinkCreateOptions());
    }

    /** Creates a new sink. */
    public StreamSinkOut create(
            final String streamId,
            final StreamSinkIn streamSinkIn,
            final StreamingSinkCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/%s/sink", streamId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), streamSinkIn, StreamSinkOut.class);
    }

    /** Get a sink by id or uid. */
    public StreamSinkOut get(final String streamId, final String sinkId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/%s/sink/%s", streamId, sinkId));
        return this.client.executeRequest("GET", url.build(), null, null, StreamSinkOut.class);
    }

    /** Update a sink. */
    public StreamSinkOut update(
            final String streamId, final String sinkId, final StreamSinkIn streamSinkIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/%s/sink/%s", streamId, sinkId));
        return this.client.executeRequest(
                "PUT", url.build(), null, streamSinkIn, StreamSinkOut.class);
    }

    /** Delete a sink. */
    public void delete(final String streamId, final String sinkId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/%s/sink/%s", streamId, sinkId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /** Partially update a sink. */
    public StreamSinkOut patch(
            final String streamId, final String sinkId, final StreamSinkPatch streamSinkPatch)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/stream/%s/sink/%s", streamId, sinkId));
        return this.client.executeRequest(
                "PATCH", url.build(), null, streamSinkPatch, StreamSinkOut.class);
    }

    /**
     * Get the sink's signing secret (only supported for http sinks)
     *
     * <p>This is used to verify the authenticity of the delivery.
     *
     * <p>For more information please refer to [the consuming webhooks
     * docs](https://docs.svix.com/consuming-webhooks/).
     */
    public SinkSecretOut getSecret(final String streamId, final String sinkId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/stream/%s/sink/%s/secret", streamId, sinkId));
        return this.client.executeRequest("GET", url.build(), null, null, SinkSecretOut.class);
    }

    /** Rotates the signing secret (only supported for http sinks). */
    public EmptyResponse rotateSecret(
            final String streamId,
            final String sinkId,
            final EndpointSecretRotateIn endpointSecretRotateIn)
            throws IOException, ApiException {
        return this.rotateSecret(
                streamId, sinkId, endpointSecretRotateIn, new StreamingSinkRotateSecretOptions());
    }

    /** Rotates the signing secret (only supported for http sinks). */
    public EmptyResponse rotateSecret(
            final String streamId,
            final String sinkId,
            final EndpointSecretRotateIn endpointSecretRotateIn,
            final StreamingSinkRotateSecretOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/stream/%s/sink/%s/secret/rotate",
                                        streamId, sinkId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                endpointSecretRotateIn,
                EmptyResponse.class);
    }

    /** Set or unset the transformation code associated with this sink. */
    public EmptyResponse transformationPartialUpdate(
            final String streamId, final String sinkId, final SinkTransformIn sinkTransformIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/stream/%s/sink/%s/transformation",
                                        streamId, sinkId));
        return this.client.executeRequest(
                "PATCH", url.build(), null, sinkTransformIn, EmptyResponse.class);
    }
}
