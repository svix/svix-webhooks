// this file is @generated
package com.svix.internalapi;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.AutoConfigSubscriptionOut;

import lombok.Getter;

import okhttp3.HttpUrl;

import java.io.IOException;

public class Autoconfig {
    private final SvixHttpClient client;

    @Getter private final AutoconfigDestination destination;
    @Getter private final AutoconfigEndpoint endpoint;

    public Autoconfig(SvixHttpClient client) {
        this.client = client;
        this.destination = new AutoconfigDestination(client);
        this.endpoint = new AutoconfigEndpoint(client);
    }

    /** Get an AutoConfig subscription, including the bound endpoint or destination if any. */
    public AutoConfigSubscriptionOut get(final String appId, final String autoconfigId)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(
                                String.format("/api/v1/app/%s/autoconfig/%s", appId, autoconfigId));
        return this.client.executeRequest(
                "GET", url.build(), null, null, AutoConfigSubscriptionOut.class);
    }
}
