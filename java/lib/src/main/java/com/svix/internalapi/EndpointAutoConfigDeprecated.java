// this file is @generated
package com.svix.internalapi;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.EndpointOut;
import com.svix.models.SubscribeIn;

import okhttp3.HttpUrl;

import java.io.IOException;

public class EndpointAutoConfigDeprecated {
    private final SvixHttpClient client;

    public EndpointAutoConfigDeprecated(SvixHttpClient client) {
        this.client = client;
    }

    /** Update an auto-config endpoint by providing endpoint details. */
    public EndpointOut update(
            final String appId, final String endpointId, final SubscribeIn subscribeIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/endpoint/%s/auto-config",
                                        appId, endpointId));
        return this.client.executeRequest("PUT", url.build(), null, subscribeIn, EndpointOut.class);
    }
}
