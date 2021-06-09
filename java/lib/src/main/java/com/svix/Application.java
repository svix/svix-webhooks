package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.ApplicationApi;
import com.svix.models.ApplicationIn;
import com.svix.models.ApplicationOut;
import com.svix.models.ListResponseApplicationOut;

public final class Application {
	private final ApplicationApi api;

	Application() {
		api = new ApplicationApi();
	}

	public ListResponseApplicationOut list(final FetchOptions options) throws ApiException {
		try {
			return api.listApplicationsApiV1AppGet(options.getIterator(), options.getLimit());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ApplicationOut create(final ApplicationIn applicationIn) throws ApiException {
		try {
			return api.createApplicationApiV1AppPost(applicationIn);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ApplicationOut get(final String appId) throws ApiException {
		try {
			return api.getApplicationApiV1AppAppIdGet(appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ApplicationOut update(final String appId, final ApplicationIn applicationIn) throws ApiException {
		try {
			return api.updateApplicationApiV1AppAppIdPut(appId, applicationIn);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void delete(final String appId) throws ApiException {
		try {
			api.deleteApplicationApiV1AppAppIdDelete(appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
