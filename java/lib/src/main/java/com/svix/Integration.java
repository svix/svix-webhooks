package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.IntegrationApi;
import com.svix.models.IntegrationIn;
import com.svix.models.IntegrationOut;
import com.svix.models.IntegrationUpdate;
import com.svix.models.IntegrationKeyOut;
import com.svix.models.ListResponseIntegrationOut;

public final class Integration {
	private final IntegrationApi api;

	public Integration() {
		api = new IntegrationApi();
	}

	public ListResponseIntegrationOut list(final String appId, final IntegrationListOptions options) throws ApiException {
		try {
			return api.v1IntegrationList(appId, options.getLimit(), options.getIterator(), options.getOrder());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public IntegrationOut create(final String appId, final IntegrationIn integrationIn) throws ApiException {
		return this.create(appId, integrationIn, new PostOptions());
	}

	public IntegrationOut create(final String appId, final IntegrationIn integrationIn, final PostOptions options) throws ApiException {
		try {
			return api.v1IntegrationCreate(appId, integrationIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public IntegrationOut get(final String appId, final String integId) throws ApiException {
		try {
			return api.v1IntegrationGet(appId, integId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public IntegrationOut update(final String appId, final String integId, final IntegrationUpdate integUpdate) throws ApiException {
		try {
			return api.v1IntegrationUpdate(appId, integId, integUpdate);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void delete(final String appId, final String integId) throws ApiException {
		try {
			api.v1IntegrationDelete(appId, integId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public IntegrationKeyOut getKey(final String appId, final String integId) throws ApiException {
		try {
			return api.v1IntegrationGetKey(appId, integId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public IntegrationKeyOut rotateKey(final String appId, final String integId) throws ApiException {
		return this.rotateKey(appId, integId, new PostOptions());
	}

	public IntegrationKeyOut rotateKey(final String appId, final String integId, final PostOptions options) throws ApiException {
		try {
			return api.v1IntegrationRotateKey(appId, integId, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
