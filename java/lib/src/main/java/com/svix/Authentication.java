package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.AuthenticationApi;
import com.svix.models.DashboardAccessOut;

public final class Authentication {
	private final AuthenticationApi api;

	Authentication() {
		api = new AuthenticationApi();
	}

	public DashboardAccessOut dashboardAccess(final String appId) throws ApiException {
		return this.dashboardAccess(appId, new PostOptions());
	}

	public DashboardAccessOut dashboardAccess(final String appId, final PostOptions options) throws ApiException {
		try {
			return api.getDashboardAccessApiV1AuthDashboardAccessAppIdPost(appId, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void logout() throws ApiException {
		this.logout(new PostOptions());
	}

	public void logout(final PostOptions options) throws ApiException {
		try {
			api.logoutApiV1AuthLogoutPost(options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
