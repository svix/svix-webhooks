package com.svix;

import com.svix.generated.ApiException;
import com.svix.generated.api.ApplicationApi;
import com.svix.model.ApplicationIn;
import com.svix.model.ApplicationOut;
import com.svix.model.ListResponseApplicationOut;

public final class Application {
	private final ApplicationApi api;

	Application() {
		api = new ApplicationApi();
	}

	public ListResponseApplicationOut list(final FetchOptions options) throws ApiException {
		return api.listApplicationsApiV1AppGet(options.getIterator(), options.getLimit());
	}

	public ApplicationOut create(final ApplicationIn applicationIn) throws ApiException {
		return api.createApplicationApiV1AppPost(applicationIn);
	}

	public ApplicationOut get(final String appId) throws ApiException {
		return api.getApplicationApiV1AppAppIdGet(appId);
	}

	public ApplicationOut update(final String appId, final ApplicationIn applicationIn) throws ApiException {
		return api.updateApplicationApiV1AppAppIdPut(appId, applicationIn);
	}

	public void delete(final String appId) throws ApiException {
		api.deleteApplicationApiV1AppAppIdDelete(appId);
	}
}
