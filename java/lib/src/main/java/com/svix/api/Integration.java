// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.IntegrationIn;
import com.svix.models.IntegrationKeyOut;
import com.svix.models.IntegrationOut;
import com.svix.models.IntegrationUpdate;
import com.svix.models.ListResponseIntegrationOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class Integration {
    private final SvixHttpClient client;

    public Integration(SvixHttpClient client) {
        this.client = client;
    }

    /** List the application's integrations. */
    public ListResponseIntegrationOut list(final String appId) throws IOException, ApiException {
        return this.list(appId, new IntegrationListOptions());
    }

    /** List the application's integrations. */
    public ListResponseIntegrationOut list(final String appId, final IntegrationListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/integration", appId));
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
                "GET", url.build(), null, null, ListResponseIntegrationOut.class);
    }

    /** Create an integration. */
    public IntegrationOut create(final String appId, final IntegrationIn integrationIn)
            throws IOException, ApiException {
        return this.create(appId, integrationIn, new IntegrationCreateOptions());
    }

    /** Create an integration. */
    public IntegrationOut create(
            final String appId,
            final IntegrationIn integrationIn,
            final IntegrationCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/integration", appId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), integrationIn, IntegrationOut.class);
    }

    /** Get an integration. */
    public IntegrationOut get(final String appId, final String integId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/integration/%s", appId, integId));
        return this.client.executeRequest("GET", url.build(), null, null, IntegrationOut.class);
    }

    /** Update an integration. */
    public IntegrationOut update(
            final String appId, final String integId, final IntegrationUpdate integrationUpdate)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/integration/%s", appId, integId));
        return this.client.executeRequest(
                "PUT", url.build(), null, integrationUpdate, IntegrationOut.class);
    }

    /** Delete an integration. */
    public void delete(final String appId, final String integId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/integration/%s", appId, integId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /**
     * Get an integration's key.
     *
     * @deprecated
     */
    @Deprecated
    public IntegrationKeyOut getKey(final String appId, final String integId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/integration/%s/key", appId, integId));
        return this.client.executeRequest("GET", url.build(), null, null, IntegrationKeyOut.class);
    }

    /** Rotate the integration's key. The previous key will be immediately revoked. */
    public IntegrationKeyOut rotateKey(final String appId, final String integId)
            throws IOException, ApiException {
        return this.rotateKey(appId, integId, new IntegrationRotateKeyOptions());
    }

    /** Rotate the integration's key. The previous key will be immediately revoked. */
    public IntegrationKeyOut rotateKey(
            final String appId, final String integId, final IntegrationRotateKeyOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/integration/%s/key/rotate",
                                        appId, integId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), null, IntegrationKeyOut.class);
    }
}
