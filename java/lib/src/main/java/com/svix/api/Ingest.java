// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.DashboardAccessOut;
import com.svix.models.IngestSourceConsumerPortalAccessIn;

import lombok.Getter;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class Ingest {
    private final SvixHttpClient client;

    @Getter private final IngestEndpoint endpoint;
    @Getter private final IngestSource source;

    public Ingest(SvixHttpClient client) {
        this.client = client;
        this.endpoint = new IngestEndpoint(client);
        this.source = new IngestSource(client);
    }

    /** Get access to the Ingest Source Consumer Portal. */
    public DashboardAccessOut dashboard(
            final String sourceId,
            final IngestSourceConsumerPortalAccessIn ingestSourceConsumerPortalAccessIn)
            throws IOException, ApiException {
        return this.dashboard(
                sourceId, ingestSourceConsumerPortalAccessIn, new IngestDashboardOptions());
    }

    /** Get access to the Ingest Source Consumer Portal. */
    public DashboardAccessOut dashboard(
            final String sourceId,
            final IngestSourceConsumerPortalAccessIn ingestSourceConsumerPortalAccessIn,
            final IngestDashboardOptions options)
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
                DashboardAccessOut.class);
    }
}
