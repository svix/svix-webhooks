// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.ApiTokenExpireIn;
import com.svix.models.ApiTokenIn;
import com.svix.models.ApiTokenOut;
import com.svix.models.ListResponseApiTokenCensoredOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class ManagementAuthentication {
    private final SvixHttpClient client;

    public ManagementAuthentication(SvixHttpClient client) {
        this.client = client;
    }

    /** List all API Tokens. */
    public ListResponseApiTokenCensoredOut listApiTokens() throws IOException, ApiException {

        return this.listApiTokens(new ManagementAuthenticationListApiTokensOptions());
    }

    /** List all API Tokens. */
    public ListResponseApiTokenCensoredOut listApiTokens(
            final ManagementAuthenticationListApiTokensOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath("/api/v1/management/authentication/api-token");
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
                "GET", url.build(), null, null, ListResponseApiTokenCensoredOut.class);
    }

    /** Create a new API Token. */
    public ApiTokenOut createApiToken(final ApiTokenIn apiTokenIn)
            throws IOException, ApiException {
        return this.createApiToken(apiTokenIn, new ManagementAuthenticationCreateApiTokenOptions());
    }

    /** Create a new API Token. */
    public ApiTokenOut createApiToken(
            final ApiTokenIn apiTokenIn,
            final ManagementAuthenticationCreateApiTokenOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath("/api/v1/management/authentication/api-token");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), apiTokenIn, ApiTokenOut.class);
    }

    /** Expire the selected API Token. */
    public void expireApiToken(final String keyId, final ApiTokenExpireIn apiTokenExpireIn)
            throws IOException, ApiException {
        this.expireApiToken(
                keyId, apiTokenExpireIn, new ManagementAuthenticationExpireApiTokenOptions());
    }

    /** Expire the selected API Token. */
    public void expireApiToken(
            final String keyId,
            final ApiTokenExpireIn apiTokenExpireIn,
            final ManagementAuthenticationExpireApiTokenOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/management/authentication/api-token/%s/expire",
                                        keyId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), apiTokenExpireIn, null);
    }
}
