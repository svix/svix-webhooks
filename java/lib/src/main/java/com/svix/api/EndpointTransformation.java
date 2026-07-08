// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.EndpointTransformationOut;
import com.svix.models.EndpointTransformationPatch;

import okhttp3.HttpUrl;

import java.io.IOException;

public class EndpointTransformation {
    private final SvixHttpClient client;

    public EndpointTransformation(SvixHttpClient client) {
        this.client = client;
    }

    /** Get the transformation code associated with this endpoint. */
    public EndpointTransformationOut get(final String appId, final String endpointId)
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
    public void patch(
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
}
