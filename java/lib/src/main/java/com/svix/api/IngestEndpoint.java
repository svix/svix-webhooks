// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.IngestEndpointHeadersIn;
import com.svix.models.IngestEndpointHeadersOut;
import com.svix.models.IngestEndpointIn;
import com.svix.models.IngestEndpointOut;
import com.svix.models.IngestEndpointSecretIn;
import com.svix.models.IngestEndpointSecretOut;
import com.svix.models.IngestEndpointTransformationOut;
import com.svix.models.IngestEndpointTransformationPatch;
import com.svix.models.IngestEndpointUpdate;
import com.svix.models.ListResponseIngestEndpointOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class IngestEndpoint {
    private final SvixHttpClient client;

    public IngestEndpoint(SvixHttpClient client) {
        this.client = client;
    }

    /** List ingest endpoints. */
    public ListResponseIngestEndpointOut list(final String sourceId)
            throws IOException, ApiException {
        return this.list(sourceId, new IngestEndpointListOptions());
    }

    /** List ingest endpoints. */
    public ListResponseIngestEndpointOut list(
            final String sourceId, final IngestEndpointListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/ingest/api/v1/source/%s/endpoint", sourceId));
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
                "GET", url.build(), null, null, ListResponseIngestEndpointOut.class);
    }

    /** Create an ingest endpoint. */
    public IngestEndpointOut create(final String sourceId, final IngestEndpointIn ingestEndpointIn)
            throws IOException, ApiException {
        return this.create(sourceId, ingestEndpointIn, new IngestEndpointCreateOptions());
    }

    /** Create an ingest endpoint. */
    public IngestEndpointOut create(
            final String sourceId,
            final IngestEndpointIn ingestEndpointIn,
            final IngestEndpointCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/ingest/api/v1/source/%s/endpoint", sourceId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                ingestEndpointIn,
                IngestEndpointOut.class);
    }

    /** Get an ingest endpoint. */
    public IngestEndpointOut get(final String sourceId, final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/ingest/api/v1/source/%s/endpoint/%s",
                                        sourceId, endpointId));
        return this.client.executeRequest("GET", url.build(), null, null, IngestEndpointOut.class);
    }

    /** Update an ingest endpoint. */
    public IngestEndpointOut update(
            final String sourceId,
            final String endpointId,
            final IngestEndpointUpdate ingestEndpointUpdate)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/ingest/api/v1/source/%s/endpoint/%s",
                                        sourceId, endpointId));
        return this.client.executeRequest(
                "PUT", url.build(), null, ingestEndpointUpdate, IngestEndpointOut.class);
    }

    /** Delete an ingest endpoint. */
    public void delete(final String sourceId, final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/ingest/api/v1/source/%s/endpoint/%s",
                                        sourceId, endpointId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /** Get the additional headers to be sent with the ingest. */
    public IngestEndpointHeadersOut getHeaders(final String sourceId, final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/ingest/api/v1/source/%s/endpoint/%s/headers",
                                        sourceId, endpointId));
        return this.client.executeRequest(
                "GET", url.build(), null, null, IngestEndpointHeadersOut.class);
    }

    /** Set the additional headers to be sent to the endpoint. */
    public void updateHeaders(
            final String sourceId,
            final String endpointId,
            final IngestEndpointHeadersIn ingestEndpointHeadersIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/ingest/api/v1/source/%s/endpoint/%s/headers",
                                        sourceId, endpointId));
        this.client.executeRequest("PUT", url.build(), null, ingestEndpointHeadersIn, null);
    }

    /**
     * Get an ingest endpoint's signing secret.
     *
     * <p>This is used to verify the authenticity of the webhook. For more information please refer
     * to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    public IngestEndpointSecretOut getSecret(final String sourceId, final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/ingest/api/v1/source/%s/endpoint/%s/secret",
                                        sourceId, endpointId));
        return this.client.executeRequest(
                "GET", url.build(), null, null, IngestEndpointSecretOut.class);
    }

    /**
     * Rotates an ingest endpoint's signing secret.
     *
     * <p>The previous secret will remain valid for the next 24 hours.
     */
    public void rotateSecret(
            final String sourceId,
            final String endpointId,
            final IngestEndpointSecretIn ingestEndpointSecretIn)
            throws IOException, ApiException {
        this.rotateSecret(
                sourceId,
                endpointId,
                ingestEndpointSecretIn,
                new IngestEndpointRotateSecretOptions());
    }

    /**
     * Rotates an ingest endpoint's signing secret.
     *
     * <p>The previous secret will remain valid for the next 24 hours.
     */
    public void rotateSecret(
            final String sourceId,
            final String endpointId,
            final IngestEndpointSecretIn ingestEndpointSecretIn,
            final IngestEndpointRotateSecretOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/ingest/api/v1/source/%s/endpoint/%s/secret/rotate",
                                        sourceId, endpointId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), ingestEndpointSecretIn, null);
    }

    /** Get the transformation code associated with this ingest endpoint. */
    public IngestEndpointTransformationOut getTransformation(
            final String sourceId, final String endpointId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/ingest/api/v1/source/%s/endpoint/%s/transformation",
                                        sourceId, endpointId));
        return this.client.executeRequest(
                "GET", url.build(), null, null, IngestEndpointTransformationOut.class);
    }

    /** Set or unset the transformation code associated with this ingest endpoint. */
    public void setTransformation(
            final String sourceId,
            final String endpointId,
            final IngestEndpointTransformationPatch ingestEndpointTransformationPatch)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/ingest/api/v1/source/%s/endpoint/%s/transformation",
                                        sourceId, endpointId));
        this.client.executeRequest(
                "PATCH", url.build(), null, ingestEndpointTransformationPatch, null);
    }
}
