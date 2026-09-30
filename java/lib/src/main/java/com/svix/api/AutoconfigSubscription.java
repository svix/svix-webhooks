// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.AutoConfigOut;
import com.svix.models.CreateAutoConfigSubscriptionIn;
import com.svix.models.RotateSubscriptionIn2;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class AutoconfigSubscription {
    private final SvixHttpClient client;

    public AutoconfigSubscription(SvixHttpClient client) {
        this.client = client;
    }

    /** Create an AutoConfig subscription. */
    public AutoConfigOut create(
            final String appId, final CreateAutoConfigSubscriptionIn createAutoConfigSubscriptionIn)
            throws IOException, ApiException {
        return this.create(
                appId, createAutoConfigSubscriptionIn, new AutoconfigSubscriptionCreateOptions());
    }

    /** Create an AutoConfig subscription. */
    public AutoConfigOut create(
            final String appId,
            final CreateAutoConfigSubscriptionIn createAutoConfigSubscriptionIn,
            final AutoconfigSubscriptionCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/autoconfig", appId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                createAutoConfigSubscriptionIn,
                AutoConfigOut.class);
    }

    /** Delete an AutoConfig subscription. This also invalidates its auth token. */
    public void delete(final String appId, final String autoconfigId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/autoconfig/%s", appId, autoconfigId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /** Rotate the auth token and signing secret for an AutoConfig subscription. */
    public AutoConfigOut rotate(
            final String appId,
            final String autoconfigId,
            final RotateSubscriptionIn2 rotateSubscriptionIn2)
            throws IOException, ApiException {
        return this.rotate(
                appId,
                autoconfigId,
                rotateSubscriptionIn2,
                new AutoconfigSubscriptionRotateOptions());
    }

    /** Rotate the auth token and signing secret for an AutoConfig subscription. */
    public AutoConfigOut rotate(
            final String appId,
            final String autoconfigId,
            final RotateSubscriptionIn2 rotateSubscriptionIn2,
            final AutoconfigSubscriptionRotateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/autoconfig/%s/rotate",
                                        appId, autoconfigId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                rotateSubscriptionIn2,
                AutoConfigOut.class);
    }
}
