package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.EnvironmentApi;
import com.svix.models.EnvironmentOut;
import com.svix.models.EnvironmentIn;

public final class Environment {
	private final EnvironmentApi api;

	Environment() {
		api = new EnvironmentApi();
	}

	public EnvironmentOut exportEnvironment() throws ApiException {
		try {
			return api.exportEnvironmentConfigurationApiV1EnvironmentExportPost(new Object());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void importEnvironment(final EnvironmentIn environmentIn) throws ApiException {
		try {
			api.importEnvironmentConfigurationApiV1EnvironmentImportPost(environmentIn);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
