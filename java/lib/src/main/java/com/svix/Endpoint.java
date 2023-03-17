package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.EndpointApi;
import com.svix.models.EndpointHeadersIn;
import com.svix.models.EndpointHeadersOut;
import com.svix.models.EndpointHeadersPatchIn;
import com.svix.models.EndpointIn;
import com.svix.models.EndpointOut;
import com.svix.models.EndpointUpdate;
import com.svix.models.EndpointSecretOut;
import com.svix.models.EndpointSecretRotateIn;
import com.svix.models.EndpointTransformationIn;
import com.svix.models.EndpointTransformationOut;
import com.svix.models.ListResponseEndpointOut;
import com.svix.models.RecoverIn;
import com.svix.models.ReplayIn;
import com.svix.models.EndpointStats;

public final class Endpoint {
	private final EndpointApi api;

	public Endpoint() {
		api = new EndpointApi();
	}

	public ListResponseEndpointOut list(final String appId, final EndpointListOptions options) throws ApiException {
		try {
			return api.listEndpointsApiV1AppAppIdEndpointGet(appId, options.getIterator(), options.getLimit(), options.getOrder(), null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointOut create(final String appId, final EndpointIn endpointIn) throws ApiException {
		return this.create(appId, endpointIn, new PostOptions());
	}

	public EndpointOut create(final String appId, final EndpointIn endpointIn, final PostOptions options) throws ApiException {
		try {
			return api.createEndpointApiV1AppAppIdEndpointPost(appId, endpointIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointOut get(final String appId, final String endpointId) throws ApiException {
		try {
			return api.getEndpointApiV1AppAppIdEndpointEndpointIdGet(endpointId, appId, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointOut update(final String appId, final String endpointId, final EndpointUpdate endpointUpdate) throws ApiException {
		try {
			return api.updateEndpointApiV1AppAppIdEndpointEndpointIdPut(endpointId, appId, endpointUpdate, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void delete(final String appId, final String endpointId) throws ApiException {
		try {
			api.deleteEndpointApiV1AppAppIdEndpointEndpointIdDelete(endpointId, appId, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointSecretOut getSecret(final String appId, final String endpointId) throws ApiException {
		try {
			return api.getEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretGet(endpointId, appId, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void rotateSecret(final String appId, final String endpointId, final EndpointSecretRotateIn endpointSecretRotateIn) throws ApiException {
		this.rotateSecret(appId, endpointId, endpointSecretRotateIn, new PostOptions());
	}

	public void rotateSecret(final String appId, final String endpointId, final EndpointSecretRotateIn endpointSecretRotateIn, final PostOptions options) throws ApiException {
		try {
			api.rotateEndpointSecretApiV1AppAppIdEndpointEndpointIdSecretRotatePost(endpointId, appId, endpointSecretRotateIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void recover(final String appId, final String endpointId, final RecoverIn recoverIn) throws ApiException {
		this.recover(appId, endpointId, recoverIn, new PostOptions());
	}

	public void recover(final String appId, final String endpointId, final RecoverIn recoverIn, final PostOptions options) throws ApiException {
		try {
			api.recoverFailedWebhooksApiV1AppAppIdEndpointEndpointIdRecoverPost(appId, endpointId, recoverIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointHeadersOut getHeaders(final String appId, final String endpointId) throws ApiException {
		try {
			return api.getEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersGet(endpointId, appId, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void updateHeaders(final String appId, final String endpointId, final EndpointHeadersIn endpointHeadersIn) throws ApiException {
		try {
			api.updateEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPut(appId, endpointId, endpointHeadersIn, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void patchHeaders(final String appId, final String endpointId, final EndpointHeadersPatchIn endpointHeadersIn) throws ApiException {
		try {
			api.patchEndpointHeadersApiV1AppAppIdEndpointEndpointIdHeadersPatch(appId, endpointId, endpointHeadersIn, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointStats getStats(final String appId, final String endpointId) throws ApiException {
		try {
			return api.getEndpointStatsApiV1AppAppIdEndpointEndpointIdStatsGet(endpointId, appId, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void replayMissing(final String appId, final String endpointId, final ReplayIn replayIn) throws ApiException {
		this.replayMissing(appId, endpointId, replayIn, new PostOptions());
	}

	public void replayMissing(final String appId, final String endpointId, final ReplayIn replayIn, final PostOptions options) throws ApiException {
		try {
			api.replayMissingWebhooksApiV1AppAppIdEndpointEndpointIdReplayMissingPostWithHttpInfo(appId, endpointId, replayIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointTransformationOut transformationGet(final String appId, final String endpointId) throws ApiException {
		try {
			return api.getEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationGet(endpointId, appId, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void transformationPartialUpdate(final String appId, final String endpointId, final EndpointTransformationIn transformationIn) throws ApiException {
		try {
			api.setEndpointTransformationApiV1AppAppIdEndpointEndpointIdTransformationPatch(appId, endpointId, transformationIn, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
