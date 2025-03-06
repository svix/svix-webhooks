// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.IngestSourceIn;
import com.svix.models.IngestSourceOut;
import com.svix.models.ListResponseIngestSourceOut;
import com.svix.models.RotateTokenOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class IngestSource {
    private final SvixHttpClient client;

    public IngestSource(SvixHttpClient client) {
        this.client = client;
    }

    /** List of all the organization's Ingest Sources. */
    public ListResponseIngestSourceOut list() throws IOException, ApiException {

        return this.list(new IngestSourceListOptions());
    }

    /** List of all the organization's Ingest Sources. */
    public ListResponseIngestSourceOut list(final IngestSourceListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/ingest/api/v1/source");
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
                "GET", url.build(), null, null, ListResponseIngestSourceOut.class);
    }

    /** Create Ingest Source. */
    public IngestSourceOut create(final IngestSourceIn ingestSourceIn)
            throws IOException, ApiException {
        return this.create(ingestSourceIn, new IngestSourceCreateOptions());
    }

    /** Create Ingest Source. */
    public IngestSourceOut create(
            final IngestSourceIn ingestSourceIn, final IngestSourceCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/ingest/api/v1/source");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), ingestSourceIn, IngestSourceOut.class);
    }

    /** Get an Ingest Source by id or uid. */
    public IngestSourceOut get(final String sourceId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/ingest/api/v1/source/%s", sourceId));
        return this.client.executeRequest("GET", url.build(), null, null, IngestSourceOut.class);
    }

    /** Update an Ingest Source. */
    public IngestSourceOut update(final String sourceId, final IngestSourceIn ingestSourceIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/ingest/api/v1/source/%s", sourceId));
        return this.client.executeRequest(
                "PUT", url.build(), null, ingestSourceIn, IngestSourceOut.class);
    }

    /** Delete an Ingest Source. */
    public void delete(final String sourceId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/ingest/api/v1/source/%s", sourceId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /**
     * Rotate the Ingest Source's Url Token.
     *
     * <p>This will rotate the ingest source's token, which is used to construct the unique
     * `ingestUrl` for the source. Previous tokens will remain valid for 48 hours after rotation.
     * The token can be rotated a maximum of three times within the 48-hour period.
     */
    public RotateTokenOut rotateToken(final String sourceId) throws IOException, ApiException {
        return this.rotateToken(sourceId, new IngestSourceRotateTokenOptions());
    }

    /**
     * Rotate the Ingest Source's Url Token.
     *
     * <p>This will rotate the ingest source's token, which is used to construct the unique
     * `ingestUrl` for the source. Previous tokens will remain valid for 48 hours after rotation.
     * The token can be rotated a maximum of three times within the 48-hour period.
     */
    public RotateTokenOut rotateToken(
            final String sourceId, final IngestSourceRotateTokenOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/ingest/api/v1/source/%s/token/rotate", sourceId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), null, RotateTokenOut.class);
    }
}
