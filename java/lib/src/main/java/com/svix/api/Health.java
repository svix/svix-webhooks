// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;

import okhttp3.HttpUrl;

import java.io.IOException;

public class Health {
    private final SvixHttpClient client;

    public Health(SvixHttpClient client) {
        this.client = client;
    }

    /** Verify the API server is up and running. */
    public void get() throws IOException, ApiException {

        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/health");
        this.client.executeRequest("GET", url.build(), null, null, null);
    }
}
