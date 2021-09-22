package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.EndpointApi;
import com.svix.models.EndpointIn;
import com.svix.models.EndpointOut;
import com.svix.models.EndpointUpdate;
import com.svix.models.EndpointSecretOut;
import com.svix.models.ListResponseEndpointOut;

public final class Endpoint {
	private final EndpointApi api;

	public Endpoint() {
		api = new EndpointApi();
	}

	public ListResponseEndpointOut list(final String appId, final EndpointListOptions options) throws ApiException {
		try {
			return api.listEndpointsApiV1AppAppIdEndpointGet(appId, options.getIterator(), options.getLimit());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointOut create(final String appId, final EndpointIn endpointIn) throws ApiException {
		try {
			return api.createEndpointApiV1AppAppIdEndpointPost(appId, endpointIn);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointOut get(final String appId, final String endpointId) throws ApiException {
		try {
			return api.getEndpointApiV1AppAppIdEndpointEndpointIdGet(endpointId, appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointOut update(final String appId, final String endpointId, final EndpointUpdate endpointUpdate) throws ApiException {
		try {
			return api.updateEndpointApiV1AppAppIdEndpointEndpointIdPut(endpointId, appId, endpointUpdate);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void delete(final String appId, final String endpointId) throws ApiException {
		try {
			api.deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(endpointId, appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointSecretOut getSecret(final String appId, final String endpointId) throws ApiException {
		try {
			return api.getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(endpointId, appId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
