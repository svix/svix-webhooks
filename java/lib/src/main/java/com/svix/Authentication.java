package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.AuthenticationApi;
import com.svix.models.AppPortalAccessIn;
import com.svix.models.AppPortalAccessOut;
import com.svix.models.DashboardAccessOut;

public final class Authentication {
	private final AuthenticationApi api;

	Authentication() {
		api = new AuthenticationApi();
	}

	public AppPortalAccessOut appPortalAccess(final String appId, final AppPortalAccessIn appPortalAccessIn) throws ApiException {
		return this.appPortalAccess(appId, appPortalAccessIn, new PostOptions());
	}

	public AppPortalAccessOut appPortalAccess(final String appId, final AppPortalAccessIn appPortalAccessIn, final PostOptions options) throws ApiException {
		try {
			return api.v1AuthenticationAppPortalAccess(appId, appPortalAccessIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public DashboardAccessOut dashboardAccess(final String appId) throws ApiException {
		return this.dashboardAccess(appId, new PostOptions());
	}

	public DashboardAccessOut dashboardAccess(final String appId, final PostOptions options) throws ApiException {
		try {
			return api.v1AuthenticationDashboardAccess(appId, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void logout() throws ApiException {
		this.logout(new PostOptions());
	}

	public void logout(final PostOptions options) throws ApiException {
		try {
			api.v1AuthenticationLogout(options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
