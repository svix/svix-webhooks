// this file is @generated
package com.svix.internalapi;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.EndpointTransformationIn;

import lombok.Getter;

import okhttp3.HttpUrl;

import java.io.IOException;

public class Endpoint {
    private final SvixHttpClient client;

    @Getter private final EndpointAutoConfig autoConfig;

    public Endpoint(SvixHttpClient client) {
        this.client = client;
        this.autoConfig = new EndpointAutoConfig(client);
    }

    /**
     * This operation was renamed to `set-transformation`.
     *
     * @deprecated
     */
    @Deprecated
    public void transformationPartialUpdate(
            final String appId,
            final String endpointId,
            final EndpointTransformationIn endpointTransformationIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/transformation",
                                        appId, endpointId));
        this.client.executeRequest("PATCH", url.build(), null, endpointTransformationIn, null);
    }
}
