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
			return api.listIntegrationsApiV1AppAppIdIntegrationGet(appId, options.getIterator(), options.getLimit());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public IntegrationOut create(final String appId, final IntegrationIn integrationIn) throws ApiException {
		try {
			return api.createIntegrationApiV1AppAppIdIntegrationPost(appId, integrationIn);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public IntegrationOut get(final String appId, final String integId) throws ApiException {
		try {
			return api.getIntegrationApiV1AppAppIdIntegrationIntegIdGet(integId, appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public IntegrationOut update(final String appId, final String integId, final IntegrationUpdate integUpdate) throws ApiException {
		try {
			return api.updateIntegrationApiV1AppAppIdIntegrationIntegIdPut(integId, appId, integUpdate);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void delete(final String appId, final String integId) throws ApiException {
		try {
			api.deleteIntegrationApiV1AppAppIdIntegrationIntegIdDelete(integId, appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public IntegrationKeyOut getKey(final String appId, final String integId) throws ApiException {
		try {
			return api.getIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyGet(integId, appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public IntegrationKeyOut rotateKey(final String appId, final String integId) throws ApiException {
		try {
			return api.rotateIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyRotatePost(integId, appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
