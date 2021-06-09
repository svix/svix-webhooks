package com.svix;

import com.svix.api.EndpointApi;
import com.svix.models.EndpointIn;
import com.svix.models.EndpointOut;
import com.svix.models.EndpointSecretOut;
import com.svix.models.ListResponseEndpointOut;

public final class Endpoint {
	private final EndpointApi api;

	public Endpoint() {
		api = new EndpointApi();
	}

	public ListResponseEndpointOut list(final String appId, final FetchOptions options) throws ApiException {
		return api.listEndpointsApiV1AppAppIdEndpointGet(appId, options.getIterator(), options.getLimit());
	}

	public EndpointOut create(final String appId, final EndpointIn endpointIn) throws ApiException {
		return api.createEndpointApiV1AppAppIdEndpointPost(appId, endpointIn);
	}

	public EndpointOut get(final String appId, final String endpointId) throws ApiException {
		return api.getEndpointApiV1AppAppIdEndpointEndpointIdGet(endpointId, appId);
	}

	public EndpointOut update(final String appId, final String endpointId, final EndpointIn endpointIn) throws ApiException {
		return api.updateEndpointApiV1AppAppIdEndpointEndpointIdPut(endpointId, appId, endpointIn);
	}

	public void delete(final String appId, final String endpointId) throws ApiException {
		api.deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(endpointId, appId);
	}

	public EndpointSecretOut getSecret(final String appId, final String endpointId) throws ApiException {
		return api.getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(endpointId, appId);
	}
}
