package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.WebhookEndpointApi;
import com.svix.models.OperationalWebhookEndpointIn;
import com.svix.models.OperationalWebhookEndpointOut;
import com.svix.models.OperationalWebhookEndpointUpdate;
import com.svix.models.OperationalWebhookEndpointSecretOut;
import com.svix.models.OperationalWebhookEndpointSecretIn;
import com.svix.models.ListResponseOperationalWebhookEndpointOut;

public final class OperationalWebhookEndpoint {
	private final WebhookEndpointApi api;

	public OperationalWebhookEndpoint() {
		api = new WebhookEndpointApi();
	}

	public ListResponseOperationalWebhookEndpointOut list(final OperationalWebhookEndpointListOptions options) throws ApiException {
		try {
			return api.listOperationalWebhookEndpoints(options.getLimit(), options.getIterator(), options.getOrder());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public OperationalWebhookEndpointOut create(final OperationalWebhookEndpointIn endpointIn) throws ApiException {
		return this.create(endpointIn, new PostOptions());
	}

	public OperationalWebhookEndpointOut create(final OperationalWebhookEndpointIn endpointIn, final PostOptions options)
			throws ApiException {
		try {
			return api.createOperationalWebhookEndpoint(endpointIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public OperationalWebhookEndpointOut get(final String endpointId) throws ApiException {
		try {
			return api.getOperationalWebhookEndpoint(endpointId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public OperationalWebhookEndpointOut update(final String endpointId, final OperationalWebhookEndpointUpdate endpointUpdate)
			throws ApiException {
		try {
			return api.updateOperationalWebhookEndpoint(endpointId, endpointUpdate);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void delete(final String endpointId) throws ApiException {
		try {
			api.deleteOperationalWebhookEndpoint(endpointId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public OperationalWebhookEndpointSecretOut getSecret(final String endpointId) throws ApiException {
		try {
			return api.getOperationalWebhookEndpointSecret(endpointId);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void rotateSecret(final String endpointId,
			final OperationalWebhookEndpointSecretIn endpointSecretIn) throws ApiException {
		this.rotateSecret(endpointId, endpointSecretIn, new PostOptions());
	}

	public void rotateSecret(final String endpointId,
			final OperationalWebhookEndpointSecretIn endpointSecretIn, final PostOptions options) throws ApiException {
		try {
			api.rotateOperationalWebhookEndpointSecret(endpointId, endpointSecretIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
