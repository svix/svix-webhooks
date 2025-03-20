// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.EnvironmentIn;
import com.svix.models.EnvironmentOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class Environment {
    private final SvixHttpClient client;

    public Environment(SvixHttpClient client) {
        this.client = client;
    }

    /** Download a JSON file containing all org-settings and event types. */
    public EnvironmentOut export() throws IOException, ApiException {

        return this.export(new EnvironmentExportOptions());
    }

    /** Download a JSON file containing all org-settings and event types. */
    public EnvironmentOut export(final EnvironmentExportOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/environment/export");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), null, EnvironmentOut.class);
    }

    /**
     * Import a configuration into the active organization.
     *
     * <p>It doesn't delete anything, only adds / updates what was passed to it.
     */
    public void import_(final EnvironmentIn environmentIn) throws IOException, ApiException {
        this.import_(environmentIn, new EnvironmentImportOptions());
    }

    /**
     * Import a configuration into the active organization.
     *
     * <p>It doesn't delete anything, only adds / updates what was passed to it.
     */
    public void import_(final EnvironmentIn environmentIn, final EnvironmentImportOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/environment/import");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        this.client.executeRequest("POST", url.build(), Headers.of(headers), environmentIn, null);
    }
}
