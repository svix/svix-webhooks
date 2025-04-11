// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;
import com.svix.exceptions.ApiException;
import com.svix.models.AppPortalAccessIn;
import com.svix.models.AppPortalAccessOut;
import com.svix.models.ApplicationTokenExpireIn;

import okhttp3.Headers;
import okhttp3.HttpUrl;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class Authentication {
    private final SvixHttpClient client;

    public Authentication(SvixHttpClient client) {
        this.client = client;
    }

    /**
     * Use this function to get magic links (and authentication codes) for connecting your users to
     * the Consumer Application Portal.
     */
    public AppPortalAccessOut appPortalAccess(
            final String appId, final AppPortalAccessIn appPortalAccessIn)
            throws IOException, ApiException {
        return this.appPortalAccess(
                appId, appPortalAccessIn, new AuthenticationAppPortalAccessOptions());
    }

    /**
     * Use this function to get magic links (and authentication codes) for connecting your users to
     * the Consumer Application Portal.
     */
    public AppPortalAccessOut appPortalAccess(
            final String appId,
            final AppPortalAccessIn appPortalAccessIn,
            final AuthenticationAppPortalAccessOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/auth/app-portal-access/%s", appId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                appPortalAccessIn,
                AppPortalAccessOut.class);
    }

    /** Expire all of the tokens associated with a specific application. */
    public void expireAll(
            final String appId, final ApplicationTokenExpireIn applicationTokenExpireIn)
            throws IOException, ApiException {
        this.expireAll(appId, applicationTokenExpireIn, new AuthenticationExpireAllOptions());
    }

    /** Expire all of the tokens associated with a specific application. */
    public void expireAll(
            final String appId,
            final ApplicationTokenExpireIn applicationTokenExpireIn,
            final AuthenticationExpireAllOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/auth/app/%s/expire-all", appId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        this.client.executeRequest(
                "POST", url.build(), Headers.of(headers), applicationTokenExpireIn, null);
    }

    /**
     * @deprecated Please use appPortalAccess instead.
     */
    @Deprecated
    public com.svix.models.DashboardAccessOut dashboardAccess(final String appId)
            throws IOException, ApiException {
        return this.dashboardAccess(appId, new AuthenticationDashboardAccessOptions());
    }

    /**
     * @deprecated Please use appPortalAccess instead.
     */
    @Deprecated
    public com.svix.models.DashboardAccessOut dashboardAccess(
            final String appId, final AuthenticationDashboardAccessOptions options)
            throws IOException, ApiException {
        HttpUrl.Builder url =
                this.client
                        .newUrlBuilder()
                        .encodedPath(String.format("/api/v1/auth/dashboard-access/%s", appId));
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        return this.client.executeRequest(
                "POST",
                url.build(),
                Headers.of(headers),
                null,
                com.svix.models.DashboardAccessOut.class);
    }

    /**
     * Logout an app token.
     *
     * <p>Trying to log out other tokens will fail.
     */
    public void logout() throws IOException, ApiException {

        this.logout(new AuthenticationLogoutOptions());
    }

    /**
     * Logout an app token.
     *
     * <p>Trying to log out other tokens will fail.
     */
    public void logout(final AuthenticationLogoutOptions options) throws IOException, ApiException {
        HttpUrl.Builder url = this.client.newUrlBuilder().encodedPath("/api/v1/auth/logout");
        Map<String, String> headers = new HashMap<>();
        if (options.idempotencyKey != null) {
            headers.put("idempotency-key", options.idempotencyKey);
        }
        this.client.executeRequest("POST", url.build(), Headers.of(headers), null, null);
    }
}
