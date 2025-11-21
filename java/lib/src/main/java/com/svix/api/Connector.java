// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.ConnectorIn;
import com.svix.models.ConnectorOut;
import com.svix.models.ConnectorPatch;
import com.svix.models.ConnectorUpdate;
import com.svix.models.ListResponseConnectorOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class Connector {
    private final SvixHttpClient client;

    public Connector(SvixHttpClient client) {
        this.client = client;
    }

    /** List all connectors for an application. */
    public ListResponseConnectorOut list() throws IOException, ApiException {

        return this.list(new ConnectorListOptions());
    }

    /** List all connectors for an application. */
    public ListResponseConnectorOut list(final ConnectorListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/connector");
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        if (options.order != null) {
            url.addQueryParameter("order", Utils.serializeQueryParam(options.order));
        }
        if (options.productType != null) {
            url.addQueryParameter("product_type", Utils.serializeQueryParam(options.productType));
        }
        return this.client.executeRequest(
                "GET", url.build(), null, null, ListResponseConnectorOut.class);
    }

    /** Create a new connector. */
    public ConnectorOut create(final ConnectorIn connectorIn) throws IOException, ApiException {
        return this.create(connectorIn, new ConnectorCreateOptions());
    }

    /** Create a new connector. */
    public ConnectorOut create(final ConnectorIn connectorIn, final ConnectorCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/connector");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), connectorIn, ConnectorOut.class);
    }

    /** Get a connector. */
    public ConnectorOut get(final String connectorId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/connector/%s", connectorId));
        return this.client.executeRequest("GET", url.build(), null, null, ConnectorOut.class);
    }

    /** Update a connector. */
    public ConnectorOut update(final String connectorId, final ConnectorUpdate connectorUpdate)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/connector/%s", connectorId));
        return this.client.executeRequest(
                "PUT", url.build(), null, connectorUpdate, ConnectorOut.class);
    }

    /** Delete a connector. */
    public void delete(final String connectorId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/connector/%s", connectorId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /** Partially update a connector. */
    public ConnectorOut patch(final String connectorId, final ConnectorPatch connectorPatch)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/connector/%s", connectorId));
        return this.client.executeRequest(
                "PATCH", url.build(), null, connectorPatch, ConnectorOut.class);
    }
}
