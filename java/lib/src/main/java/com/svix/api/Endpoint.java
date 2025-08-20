// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.EndpointHeadersIn;
import com.svix.models.EndpointHeadersOut;
import com.svix.models.EndpointHeadersPatchIn;
import com.svix.models.EndpointIn;
import com.svix.models.EndpointOut;
import com.svix.models.EndpointPatch;
import com.svix.models.EndpointSecretOut;
import com.svix.models.EndpointSecretRotateIn;
import com.svix.models.EndpointStats;
import com.svix.models.EndpointTransformationIn;
import com.svix.models.EndpointTransformationOut;
import com.svix.models.EndpointTransformationPatch;
import com.svix.models.EndpointUpdate;
import com.svix.models.EventExampleIn;
import com.svix.models.ListResponseEndpointOut;
import com.svix.models.MessageOut;
import com.svix.models.RecoverIn;
import com.svix.models.RecoverOut;
import com.svix.models.ReplayIn;
import com.svix.models.ReplayOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class Endpoint {
    private final SvixHttpClient client;

    public Endpoint(SvixHttpClient client) {
        this.client = client;
    }

    /** List the application's endpoints. */
    public ListResponseEndpointOut list(final String appId) throws IOException, ApiException {
        return this.list(appId, new EndpointListOptions());
    }

    /** List the application's endpoints. */
    public ListResponseEndpointOut list(final String appId, final EndpointListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/endpoint", appId));
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
                "GET", url.build(), null, null, ListResponseEndpointOut.class);
    }

    /**
     * Create a new endpoint for the application.
     *
     * <p>When `secret` is `null` the secret is automatically generated (recommended).
     */
    public EndpointOut create(final String appId, final EndpointIn endpointIn)
            throws IOException, ApiException {
        return this.create(appId, endpointIn, new EndpointCreateOptions());
    }

    /**
     * Create a new endpoint for the application.
     *
     * <p>When `secret` is `null` the secret is automatically generated (recommended).
     */
    public EndpointOut create(
            final String appId, final EndpointIn endpointIn, final EndpointCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/endpoint", appId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), endpointIn, EndpointOut.class);
    }

    /** Get an endpoint. */
    public EndpointOut get(final String appId, final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/endpoint/%s", appId, endpointId));
        return this.client.executeRequest("GET", url.build(), null, null, EndpointOut.class);
    }

    /** Update an endpoint. */
    public EndpointOut update(
            final String appId, final String endpointId, final EndpointUpdate endpointUpdate)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/endpoint/%s", appId, endpointId));
        return this.client.executeRequest(
                "PUT", url.build(), null, endpointUpdate, EndpointOut.class);
    }

    /** Delete an endpoint. */
    public void delete(final String appId, final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/endpoint/%s", appId, endpointId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /** Partially update an endpoint. */
    public EndpointOut patch(
            final String appId, final String endpointId, final EndpointPatch endpointPatch)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/endpoint/%s", appId, endpointId));
        return this.client.executeRequest(
                "PATCH", url.build(), null, endpointPatch, EndpointOut.class);
    }

    /** Get the additional headers to be sent with the webhook. */
    public EndpointHeadersOut getHeaders(final String appId, final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/headers", appId, endpointId));
        return this.client.executeRequest("GET", url.build(), null, null, EndpointHeadersOut.class);
    }

    /** Set the additional headers to be sent with the webhook. */
    public void updateHeaders(
            final String appId, final String endpointId, final EndpointHeadersIn endpointHeadersIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/headers", appId, endpointId));
        this.client.executeRequest("PUT", url.build(), null, endpointHeadersIn, null);
    }

    /** Partially set the additional headers to be sent with the webhook. */
    public void patchHeaders(
            final String appId,
            final String endpointId,
            final EndpointHeadersPatchIn endpointHeadersPatchIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/headers", appId, endpointId));
        this.client.executeRequest("PATCH", url.build(), null, endpointHeadersPatchIn, null);
    }

    /**
     * Resend all failed messages since a given time.
     *
     * <p>Messages that were sent successfully, even if failed initially, are not resent.
     */
    public RecoverOut recover(
            final String appId, final String endpointId, final RecoverIn recoverIn)
            throws IOException, ApiException {
        return this.recover(appId, endpointId, recoverIn, new EndpointRecoverOptions());
    }

    /**
     * Resend all failed messages since a given time.
     *
     * <p>Messages that were sent successfully, even if failed initially, are not resent.
     */
    public RecoverOut recover(
            final String appId,
            final String endpointId,
            final RecoverIn recoverIn,
            final EndpointRecoverOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/recover", appId, endpointId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), recoverIn, RecoverOut.class);
    }

    /**
     * Replays messages to the endpoint.
     *
     * <p>Only messages that were created after `since` will be sent. Messages that were previously
     * sent to the endpoint are not resent.
     */
    public ReplayOut replayMissing(
            final String appId, final String endpointId, final ReplayIn replayIn)
            throws IOException, ApiException {
        return this.replayMissing(appId, endpointId, replayIn, new EndpointReplayMissingOptions());
    }

    /**
     * Replays messages to the endpoint.
     *
     * <p>Only messages that were created after `since` will be sent. Messages that were previously
     * sent to the endpoint are not resent.
     */
    public ReplayOut replayMissing(
            final String appId,
            final String endpointId,
            final ReplayIn replayIn,
            final EndpointReplayMissingOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/replay-missing",
                                        appId, endpointId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), replayIn, ReplayOut.class);
    }

    /**
     * Get the endpoint's signing secret.
     *
     * <p>This is used to verify the authenticity of the webhook. For more information please refer
     * to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    public EndpointSecretOut getSecret(final String appId, final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/secret", appId, endpointId));
        return this.client.executeRequest("GET", url.build(), null, null, EndpointSecretOut.class);
    }

    /**
     * Rotates the endpoint's signing secret.
     *
     * <p>The previous secret will remain valid for the next 24 hours.
     */
    public void rotateSecret(
            final String appId,
            final String endpointId,
            final EndpointSecretRotateIn endpointSecretRotateIn)
            throws IOException, ApiException {
        this.rotateSecret(
                appId, endpointId, endpointSecretRotateIn, new EndpointRotateSecretOptions());
    }

    /**
     * Rotates the endpoint's signing secret.
     *
     * <p>The previous secret will remain valid for the next 24 hours.
     */
    public void rotateSecret(
            final String appId,
            final String endpointId,
            final EndpointSecretRotateIn endpointSecretRotateIn,
            final EndpointRotateSecretOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/secret/rotate",
                                        appId, endpointId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), endpointSecretRotateIn, null);
    }

    /** Send an example message for an event. */
    public MessageOut sendExample(
            final String appId, final String endpointId, final EventExampleIn eventExampleIn)
            throws IOException, ApiException {
        return this.sendExample(
                appId, endpointId, eventExampleIn, new EndpointSendExampleOptions());
    }

    /** Send an example message for an event. */
    public MessageOut sendExample(
            final String appId,
            final String endpointId,
            final EventExampleIn eventExampleIn,
            final EndpointSendExampleOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/send-example",
                                        appId, endpointId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), eventExampleIn, MessageOut.class);
    }

    /** Get basic statistics for the endpoint. */
    public EndpointStats getStats(final String appId, final String endpointId)
            throws IOException, ApiException {
        return this.getStats(appId, endpointId, new EndpointGetStatsOptions());
    }

    /** Get basic statistics for the endpoint. */
    public EndpointStats getStats(
            final String appId, final String endpointId, final EndpointGetStatsOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/stats", appId, endpointId));
        if (options.since != null) {
            url.addQueryParameter("since", Utils.serializeQueryParam(options.since));
        }
        if (options.until != null) {
            url.addQueryParameter("until", Utils.serializeQueryParam(options.until));
        }
        return this.client.executeRequest("GET", url.build(), null, null, EndpointStats.class);
    }

    /** Get the transformation code associated with this endpoint. */
    public EndpointTransformationOut transformationGet(final String appId, final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/transformation",
                                        appId, endpointId));
        return this.client.executeRequest(
                "GET", url.build(), null, null, EndpointTransformationOut.class);
    }

    /** Set or unset the transformation code associated with this endpoint. */
    public void patchTransformation(
            final String appId,
            final String endpointId,
            final EndpointTransformationPatch endpointTransformationPatch)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/transformation",
                                        appId, endpointId));
        this.client.executeRequest("PATCH", url.build(), null, endpointTransformationPatch, null);
    }

    /**
     * This operation was renamed to `set-transformation`.
     *
     * @deprecated
     */
    @Deprecated
    public void transformationPartialUpdate(
            final String appId,
            final String endpointId,
            final EndpointTransformationIn endpointTransformationIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/transformation",
                                        appId, endpointId));
        this.client.executeRequest("PATCH", url.build(), null, endpointTransformationIn, null);
    }
}
