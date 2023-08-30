package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.EventTypeApi;
import com.svix.models.EventTypeImportOpenApiIn;
import com.svix.models.EventTypeImportOpenApiOut;
import com.svix.models.EventTypeIn;
import com.svix.models.EventTypeOut;
import com.svix.models.EventTypePatch;
import com.svix.models.EventTypeUpdate;
import com.svix.models.ListResponseEventTypeOut;

public final class EventType {
	private final EventTypeApi api;

	public EventType() {
		api = new EventTypeApi();
	}

	public ListResponseEventTypeOut list(final EventTypeListOptions options) throws ApiException {
		try {
			return api.v1EventTypeList(options.getLimit(), options.getIterator(), null, options.getIncludeArchived(), options.getWithContent());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EventTypeOut create(final EventTypeIn eventTypeIn) throws ApiException {
		return this.create(eventTypeIn, new PostOptions());
	}

	public EventTypeOut create(final EventTypeIn eventTypeIn, final PostOptions options) throws ApiException {
		try {
			return api.v1EventTypeCreate(eventTypeIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EventTypeOut get(final String eventTypeName) throws ApiException {
		try {
			return api.v1EventTypeGet(eventTypeName);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EventTypeOut update(final String eventTypeName, final EventTypeUpdate eventTypeUpdate) throws ApiException {
		try {
			return api.v1EventTypeUpdate(eventTypeName, eventTypeUpdate);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EventTypeOut patch(final String eventTypeName, final EventTypePatch eventTypePatch) throws ApiException {
		try {
			return api.v1EventTypePatch(eventTypeName, eventTypePatch);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void delete(final String eventTypeName) throws ApiException {
		try {
			api.v1EventTypeDelete(eventTypeName, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EventTypeImportOpenApiOut importOpenApi(final EventTypeImportOpenApiIn eventTypeImportOpenApiIn) throws ApiException {
		try {
			return this.importOpenApi(eventTypeImportOpenApiIn, new PostOptions());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EventTypeImportOpenApiOut importOpenApi(final EventTypeImportOpenApiIn eventTypeImportOpenApiIn, final PostOptions options) throws ApiException {
		try {
			return api.v1EventTypeImportOpenapi(eventTypeImportOpenApiIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
