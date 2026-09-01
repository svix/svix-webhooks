// this file is @generated
package com.svix.internalapi;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.DestinationIn;
import com.svix.models.DestinationOut;

import okhttp3.HttpUrl;

import java.io.IOException;

public class DestinationAutoconfig {
    private final SvixHttpClient client;

    public DestinationAutoconfig(SvixHttpClient client) {
        this.client = client;
    }

    /** Create or update the destination for an AutoConfig subscription. */
    public DestinationOut subscribe(
            final String appId, final String autoconfigId, final DestinationIn destinationIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format(
                                        "/api/v1/app/%s/autoconfig/%s/destination",
                                        appId, autoconfigId));
        return this.client.executeRequest(
                "PUT", url.build(), null, destinationIn, DestinationOut.class);
    }
}
