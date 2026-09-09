// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.DestinationIn;
import com.svix.models.DestinationOut;
import com.svix.models.DestinationPatch;
import com.svix.models.ListResponseDestinationOut;

import lombok.Getter;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class Destination {
    private final SvixHttpClient client;

    @Getter private final DestinationTransformation transformation;

    public Destination(SvixHttpClient client) {
        this.client = client;
        this.transformation = new DestinationTransformation(client);
    }

    /** List of all the application's destinations. */
    public ListResponseDestinationOut list(final String appId) throws IOException, ApiException {
        return this.list(appId, new DestinationListOptions());
    }

    /** List of all the application's destinations. */
    public ListResponseDestinationOut list(final String appId, final DestinationListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/destination", appId));
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
                "GET", url.build(), null, null, ListResponseDestinationOut.class);
    }

    /** Creates a new destination. */
    public DestinationOut create(final String appId, final DestinationIn destinationIn)
            throws IOException, ApiException {
        return this.create(appId, destinationIn, new DestinationCreateOptions());
    }

    /** Creates a new destination. */
    public DestinationOut create(
            final String appId,
            final DestinationIn destinationIn,
            final DestinationCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/app/%s/destination", appId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), destinationIn, DestinationOut.class);
    }

    /** Get a destination by id or uid. */
    public DestinationOut get(final String appId, final String destinationId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/destination/%s", appId, destinationId));
        return this.client.executeRequest("GET", url.build(), null, null, DestinationOut.class);
    }

    /** Create or update a destination. */
    public DestinationOut upsert(
            final String appId, final String destinationId, final DestinationIn destinationIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/destination/%s", appId, destinationId));
        return this.client.executeRequest(
                "PUT", url.build(), null, destinationIn, DestinationOut.class);
    }

    /** Delete a destination. */
    public void delete(final String appId, final String destinationId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/destination/%s", appId, destinationId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /** Partially update a destination. */
    public DestinationOut patch(
            final String appId, final String destinationId, final DestinationPatch destinationPatch)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/destination/%s", appId, destinationId));
        return this.client.executeRequest(
                "PATCH", url.build(), null, destinationPatch, DestinationOut.class);
    }
}
