// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.IngestEndpointTransformationOut;
import com.svix.models.IngestEndpointTransformationPatch;

import okhttp3.HttpUrl;

import java.io.IOException;

public class IngestEndpointTransformation {
    private final SvixHttpClient client;

    public IngestEndpointTransformation(SvixHttpClient client) {
        this.client = client;
    }

    /** Get the transformation code associated with this ingest endpoint. */
    public IngestEndpointTransformationOut transformation(
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
    public void patch(
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
