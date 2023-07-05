package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.EndpointApi;
import com.svix.models.EndpointHeadersIn;
import com.svix.models.EndpointHeadersOut;
import com.svix.models.EndpointHeadersPatchIn;
import com.svix.models.EndpointIn;
import com.svix.models.EndpointOut;
import com.svix.models.EndpointPatch;
import com.svix.models.EndpointUpdate;
import com.svix.models.EndpointSecretOut;
import com.svix.models.EndpointSecretRotateIn;
import com.svix.models.EndpointTransformationIn;
import com.svix.models.EndpointTransformationOut;
import com.svix.models.EventExampleIn;
import com.svix.models.ListResponseEndpointOut;
import com.svix.models.MessageOut;
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
			return api.v1EndpointList(appId, options.getLimit(), options.getIterator(), options.getOrder());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointOut create(final String appId, final EndpointIn endpointIn) throws ApiException {
		return this.create(appId, endpointIn, new PostOptions());
	}

	public EndpointOut create(final String appId, final EndpointIn endpointIn, final PostOptions options)
			throws ApiException {
		try {
			return api.v1EndpointCreate(appId, endpointIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointOut get(final String appId, final String endpointId) throws ApiException {
		try {
			return api.v1EndpointGet(appId, endpointId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointOut update(final String appId, final String endpointId, final EndpointUpdate endpointUpdate)
			throws ApiException {
		try {
			return api.v1EndpointUpdate(appId, endpointId, endpointUpdate);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointOut patch(final String appId, final String endpointId, final EndpointPatch endpointPatch)
			throws ApiException {
		try {
			return api.v1EndpointPatch(appId, endpointId, endpointPatch);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void delete(final String appId, final String endpointId) throws ApiException {
		try {
			api.v1EndpointDelete(appId, endpointId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointSecretOut getSecret(final String appId, final String endpointId) throws ApiException {
		try {
			return api.v1EndpointGetSecret(appId, endpointId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void rotateSecret(final String appId, final String endpointId,
			final EndpointSecretRotateIn endpointSecretRotateIn) throws ApiException {
		this.rotateSecret(appId, endpointId, endpointSecretRotateIn, new PostOptions());
	}

	public void rotateSecret(final String appId, final String endpointId,
			final EndpointSecretRotateIn endpointSecretRotateIn, final PostOptions options) throws ApiException {
		try {
			api.v1EndpointRotateSecret(appId, endpointId, endpointSecretRotateIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void recover(final String appId, final String endpointId, final RecoverIn recoverIn) throws ApiException {
		this.recover(appId, endpointId, recoverIn, new PostOptions());
	}

	public void recover(final String appId, final String endpointId, final RecoverIn recoverIn,
			final PostOptions options) throws ApiException {
		try {
			api.v1EndpointRecover(appId, endpointId, recoverIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointHeadersOut getHeaders(final String appId, final String endpointId) throws ApiException {
		try {
			return api.v1EndpointGetHeaders(appId, endpointId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void updateHeaders(final String appId, final String endpointId, final EndpointHeadersIn endpointHeadersIn)
			throws ApiException {
		try {
			api.v1EndpointUpdateHeaders(appId, endpointId, endpointHeadersIn);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void patchHeaders(final String appId, final String endpointId,
			final EndpointHeadersPatchIn endpointHeadersIn) throws ApiException {
		try {
			api.v1EndpointPatchHeaders(appId, endpointId, endpointHeadersIn);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointStats getStats(final String appId, final String endpointId) throws ApiException {
		return getStats(appId, endpointId, new EndpointStatsOptions());
	}

	public EndpointStats getStats(final String appId, final String endpointId, final EndpointStatsOptions options)
			throws ApiException {
		try {
			return api.v1EndpointGetStats(appId, endpointId, options.getSince(), options.getUntil());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void replayMissing(final String appId, final String endpointId, final ReplayIn replayIn)
			throws ApiException {
		this.replayMissing(appId, endpointId, replayIn, new PostOptions());
	}

	public void replayMissing(final String appId, final String endpointId, final ReplayIn replayIn,
			final PostOptions options) throws ApiException {
		try {
			api.v1EndpointReplay(appId, endpointId, replayIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EndpointTransformationOut transformationGet(final String appId, final String endpointId)
			throws ApiException {
		try {
			return api.v1EndpointTransformationGet(appId, endpointId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void transformationPartialUpdate(final String appId, final String endpointId,
			final EndpointTransformationIn transformationIn) throws ApiException {
		try {
			api.v1EndpointTransformationPartialUpdate(appId, endpointId, transformationIn);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public MessageOut sendExample(final String appId, final String endpointId, final EventExampleIn eventExampleIn)
			throws ApiException {
		return this.sendExample(appId, endpointId, eventExampleIn, new PostOptions());
	}

	public MessageOut sendExample(final String appId, final String endpointId, final EventExampleIn eventExampleIn,
			final PostOptions options) throws ApiException {
		try {
			return api.v1EndpointSendExample(appId, endpointId, eventExampleIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
