// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.AppPortalAccessOut;
import com.svix.models.IngestSourceConsumerPortalAccessIn;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class IngestAuthentication {
    private final SvixHttpClient client;

    public IngestAuthentication(SvixHttpClient client) {
        this.client = client;
    }

    /** Get access to the Ingest Source Consumer Portal. */
    public AppPortalAccessOut consumerPortalAccess(
            final String sourceId,
            final IngestSourceConsumerPortalAccessIn ingestSourceConsumerPortalAccessIn)
            throws IOException, ApiException {
        return this.consumerPortalAccess(
                sourceId,
                ingestSourceConsumerPortalAccessIn,
                new IngestAuthenticationConsumerPortalAccessOptions());
    }

    /** Get access to the Ingest Source Consumer Portal. */
    public AppPortalAccessOut consumerPortalAccess(
            final String sourceId,
            final IngestSourceConsumerPortalAccessIn ingestSourceConsumerPortalAccessIn,
            final IngestAuthenticationConsumerPortalAccessOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/ingest/api/v1/source/%s/dashboard", sourceId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                ingestSourceConsumerPortalAccessIn,
                AppPortalAccessOut.class);
    }
}
