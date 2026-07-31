// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.EmptyResponse;
import com.svix.models.SinkTransformIn;
import com.svix.models.SinkTransformationOut;

import okhttp3.HttpUrl;

import java.io.IOException;

public class StreamingSinkTransformation {
    private final SvixHttpClient client;

    public StreamingSinkTransformation(SvixHttpClient client) {
        this.client = client;
    }

    /** Get the transformation code associated with this sink. */
    public SinkTransformationOut get(final String streamId, final String sinkId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/stream/%s/sink/%s/transformation",
                                        streamId, sinkId));
        return this.client.executeRequest(
                "GET", url.build(), null, null, SinkTransformationOut.class);
    }

    /** Set or unset the transformation code associated with this sink. */
    public EmptyResponse patch(
            final String streamId, final String sinkId, final SinkTransformIn sinkTransformIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/stream/%s/sink/%s/transformation",
                                        streamId, sinkId));
        return this.client.executeRequest(
                "PATCH", url.build(), null, sinkTransformIn, EmptyResponse.class);
    }
}
