// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.ListResponseOperationalWebhookEndpointOut;
import com.svix.models.OperationalWebhookEndpointHeadersIn;
import com.svix.models.OperationalWebhookEndpointHeadersOut;
import com.svix.models.OperationalWebhookEndpointIn;
import com.svix.models.OperationalWebhookEndpointOut;
import com.svix.models.OperationalWebhookEndpointSecretIn;
import com.svix.models.OperationalWebhookEndpointSecretOut;
import com.svix.models.OperationalWebhookEndpointUpdate;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class OperationalWebhookEndpoint {
    private final SvixHttpClient client;

    public OperationalWebhookEndpoint(SvixHttpClient client) {
        this.client = client;
    }

    /** List operational webhook endpoints. */
    public ListResponseOperationalWebhookEndpointOut list() throws IOException, ApiException {

        return this.list(new OperationalWebhookEndpointListOptions());
    }

    /** List operational webhook endpoints. */
    public ListResponseOperationalWebhookEndpointOut list(
            final OperationalWebhookEndpointListOptions options) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint");
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
                "GET", url.build(), null, null, ListResponseOperationalWebhookEndpointOut.class);
    }

    /** Create an operational webhook endpoint. */
    public OperationalWebhookEndpointOut create(
            final OperationalWebhookEndpointIn operationalWebhookEndpointIn)
            throws IOException, ApiException {
        return this.create(
                operationalWebhookEndpointIn, new OperationalWebhookEndpointCreateOptions());
    }

    /** Create an operational webhook endpoint. */
    public OperationalWebhookEndpointOut create(
            final OperationalWebhookEndpointIn operationalWebhookEndpointIn,
            final OperationalWebhookEndpointCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client.newUrlBuilder().encodedPath("/api/v1/operational-webhook/endpoint");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                operationalWebhookEndpointIn,
                OperationalWebhookEndpointOut.class);
    }

    /** Get an operational webhook endpoint. */
    public OperationalWebhookEndpointOut get(final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/operational-webhook/endpoint/%s", endpointId));
        return this.client.executeRequest(
                "GET", url.build(), null, null, OperationalWebhookEndpointOut.class);
    }

    /** Update an operational webhook endpoint. */
    public OperationalWebhookEndpointOut update(
            final String endpointId,
            final OperationalWebhookEndpointUpdate operationalWebhookEndpointUpdate)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/operational-webhook/endpoint/%s", endpointId));
        return this.client.executeRequest(
                "PUT",
                url.build(),
                null,
                operationalWebhookEndpointUpdate,
                OperationalWebhookEndpointOut.class);
    }

    /** Delete an operational webhook endpoint. */
    public void delete(final String endpointId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/operational-webhook/endpoint/%s", endpointId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /** Get the additional headers to be sent with the operational webhook. */
    public OperationalWebhookEndpointHeadersOut getHeaders(final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/operational-webhook/endpoint/%s/headers",
                                        endpointId));
        return this.client.executeRequest(
                "GET", url.build(), null, null, OperationalWebhookEndpointHeadersOut.class);
    }

    /** Set the additional headers to be sent with the operational webhook. */
    public void updateHeaders(
            final String endpointId,
            final OperationalWebhookEndpointHeadersIn operationalWebhookEndpointHeadersIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/operational-webhook/endpoint/%s/headers",
                                        endpointId));
        this.client.executeRequest(
                "PUT", url.build(), null, operationalWebhookEndpointHeadersIn, null);
    }

    /**
     * Get an operational webhook endpoint's signing secret.
     *
     * <p>This is used to verify the authenticity of the webhook. For more information please refer
     * to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    public OperationalWebhookEndpointSecretOut getSecret(final String endpointId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/operational-webhook/endpoint/%s/secret",
                                        endpointId));
        return this.client.executeRequest(
                "GET", url.build(), null, null, OperationalWebhookEndpointSecretOut.class);
    }

    /**
     * Rotates an operational webhook endpoint's signing secret.
     *
     * <p>The previous secret will remain valid for the next 24 hours.
     */
    public void rotateSecret(
            final String endpointId,
            final OperationalWebhookEndpointSecretIn operationalWebhookEndpointSecretIn)
            throws IOException, ApiException {
        this.rotateSecret(
                endpointId,
                operationalWebhookEndpointSecretIn,
                new OperationalWebhookEndpointRotateSecretOptions());
    }

    /**
     * Rotates an operational webhook endpoint's signing secret.
     *
     * <p>The previous secret will remain valid for the next 24 hours.
     */
    public void rotateSecret(
            final String endpointId,
            final OperationalWebhookEndpointSecretIn operationalWebhookEndpointSecretIn,
            final OperationalWebhookEndpointRotateSecretOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/operational-webhook/endpoint/%s/secret/rotate",
                                        endpointId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), operationalWebhookEndpointSecretIn, null);
    }
}
