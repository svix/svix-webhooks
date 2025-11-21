// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.Utils;
import com.svix.exceptions.ApiException;
import com.svix.models.ApplicationIn;
import com.svix.models.ApplicationOut;
import com.svix.models.ApplicationPatch;
import com.svix.models.ListResponseApplicationOut;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class Application {
    private final SvixHttpClient client;

    public Application(SvixHttpClient client) {
        this.client = client;
    }

    /** List of all the organization's applications. */
    public ListResponseApplicationOut list() throws IOException, ApiException {

        return this.list(new ApplicationListOptions());
    }

    /** List of all the organization's applications. */
    public ListResponseApplicationOut list(final ApplicationListOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/app");
        if (options.excludeAppsWithNoEndpoints != null) {
            url.addQueryParameter(
                    "exclude_apps_with_no_endpoints",
                    Utils.serializeQueryParam(options.excludeAppsWithNoEndpoints));
        }
        if (options.excludeAppsWithDisabledEndpoints != null) {
            url.addQueryParameter(
                    "exclude_apps_with_disabled_endpoints",
                    Utils.serializeQueryParam(options.excludeAppsWithDisabledEndpoints));
        }
        if (options.limit != null) {
            url.addQueryParameter("limit", Utils.serializeQueryParam(options.limit));
        }
        if (options.iterator != null) {
            url.addQueryParameter("iterator", options.iterator);
        }
        if (options.order != null) {
            url.addQueryParameter("order", Utils.serializeQueryParam(options.order));
        }
        return this.client.executeRequest(
                "GET", url.build(), null, null, ListResponseApplicationOut.class);
    }

    /** Create a new application. */
    public ApplicationOut create(final ApplicationIn applicationIn)
            throws IOException, ApiException {
        return this.create(applicationIn, new ApplicationCreateOptions());
    }

    /** Create a new application. */
    public ApplicationOut create(
            final ApplicationIn applicationIn, final ApplicationCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/app");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), applicationIn, ApplicationOut.class);
    }

    /** Create a new application. */
    public ApplicationOut getOrCreate(final ApplicationIn applicationIn)
            throws IOException, ApiException {
        return this.getOrCreate(applicationIn, new ApplicationCreateOptions());
    }

    /** Create a new application. */
    public ApplicationOut getOrCreate(
            final ApplicationIn applicationIn, final ApplicationCreateOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/app");
        url.addQueryParameter("get_if_exists", "true");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), applicationIn, ApplicationOut.class);
    }

    /** Get an application. */
    public ApplicationOut get(final String appId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client.newUrlBuilder().encodedPath(String.format("/api/v1/app/%s", appId));
        return this.client.executeRequest("GET", url.build(), null, null, ApplicationOut.class);
    }

    /** Update an application. */
    public ApplicationOut update(final String appId, final ApplicationIn applicationIn)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client.newUrlBuilder().encodedPath(String.format("/api/v1/app/%s", appId));
        return this.client.executeRequest(
                "PUT", url.build(), null, applicationIn, ApplicationOut.class);
    }

    /** Delete an application. */
    public void delete(final String appId) throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client.newUrlBuilder().encodedPath(String.format("/api/v1/app/%s", appId));
        this.client.executeRequest("DELETE", url.build(), null, null, null);
    }

    /** Partially update an application. */
    public ApplicationOut patch(final String appId, final ApplicationPatch applicationPatch)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client.newUrlBuilder().encodedPath(String.format("/api/v1/app/%s", appId));
        return this.client.executeRequest(
                "PATCH", url.build(), null, applicationPatch, ApplicationOut.class);
    }
}
