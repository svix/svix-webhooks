// this file is @generated
package com.svix.internalapi;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.EndpointIn;
import com.svix.models.EndpointOut;

import okhttp3.HttpUrl;

import java.io.IOException;

public class EndpointAutoconfig {
    private final SvixHttpClient client;

    public EndpointAutoconfig(SvixHttpClient client) {
        this.client = client;
    }

    /** Create or update the HTTP endpoint for an AutoConfig subscription. */
    public EndpointOut subscribe(
            final String appId, final String autoconfigId, final EndpointIn endpointIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/autoconfig/%s/endpoint",
                                        appId, autoconfigId));
        return this.client.executeRequest("PUT", url.build(), null, endpointIn, EndpointOut.class);
    }
}
