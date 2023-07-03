package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.ApplicationApi;
import com.svix.models.ApplicationIn;
import com.svix.models.ApplicationOut;
import com.svix.models.ApplicationPatch;
import com.svix.models.ListResponseApplicationOut;

public final class Application {
	private final ApplicationApi api;

	Application() {
		api = new ApplicationApi();
	}

	public ListResponseApplicationOut list(final ApplicationListOptions options) throws ApiException {
		try {
			return api.v1ApplicationList(options.getLimit(), options.getIterator(), options.getOrder());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ApplicationOut create(final ApplicationIn applicationIn) throws ApiException {
		return this.create(applicationIn, new PostOptions());
	}

	public ApplicationOut create(final ApplicationIn applicationIn, final PostOptions options) throws ApiException {
		try {
			return api.v1ApplicationCreate(applicationIn, null, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ApplicationOut getOrCreate(final ApplicationIn applicationIn) throws ApiException {
		return this.getOrCreate(applicationIn, new PostOptions());
	}

	public ApplicationOut getOrCreate(final ApplicationIn applicationIn, final PostOptions options) throws ApiException {
		try {
			return api.v1ApplicationCreate(applicationIn, true, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ApplicationOut get(final String appId) throws ApiException {
		try {
			return api.v1ApplicationGet(appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ApplicationOut update(final String appId, final ApplicationIn applicationIn) throws ApiException {
		try {
			return api.v1ApplicationUpdate(appId, applicationIn);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public ApplicationOut patch(final String appId, final ApplicationPatch applicationPatch) throws ApiException {
		try {
			return api.v1ApplicationPatch(appId, applicationPatch);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void delete(final String appId) throws ApiException {
		try {
			api.v1ApplicationDelete(appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
