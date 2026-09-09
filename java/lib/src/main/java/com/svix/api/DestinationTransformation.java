// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.DestinationTransformIn;
import com.svix.models.DestinationTransformationOut;
import com.svix.models.EmptyResponse;

import okhttp3.HttpUrl;

import java.io.IOException;

public class DestinationTransformation {
    private final SvixHttpClient client;

    public DestinationTransformation(SvixHttpClient client) {
        this.client = client;
    }

    /** Get the transformation code associated with this destination. */
    public DestinationTransformationOut get(final String appId, final String destinationId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/destination/%s/transformation",
                                        appId, destinationId));
        return this.client.executeRequest(
                "GET", url.build(), null, null, DestinationTransformationOut.class);
    }

    /** Set or unset the transformation code associated with this destination. */
    public EmptyResponse patch(
            final String appId,
            final String destinationId,
            final DestinationTransformIn destinationTransformIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/destination/%s/transformation",
                                        appId, destinationId));
        return this.client.executeRequest(
                "PATCH", url.build(), null, destinationTransformIn, EmptyResponse.class);
    }
}
