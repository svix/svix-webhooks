package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.EventTypeApi;
import com.svix.models.EventTypeIn;
import com.svix.models.EventTypeOut;
import com.svix.models.EventTypeUpdate;
import com.svix.models.ListResponseEventTypeOut;

public final class EventType {
	private final EventTypeApi api;

	public EventType() {
		api = new EventTypeApi();
	}

	public ListResponseEventTypeOut list(final EventTypeListOptions options) throws ApiException {
		try {
			return api.listEventTypesApiV1EventTypeGet(options.getIterator(), options.getLimit(), options.getWithContent(), options.getIncludeArchived(), null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EventTypeOut create(final EventTypeIn eventTypeIn) throws ApiException {
		return this.create(eventTypeIn, new PostOptions());
	}

	public EventTypeOut create(final EventTypeIn eventTypeIn, final PostOptions options) throws ApiException {
		try {
			return api.createEventTypeApiV1EventTypePost(eventTypeIn, options.getIdempotencyKey());
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EventTypeOut get(final String eventTypeName) throws ApiException {
		try {
			return api.getEventTypeApiV1EventTypeEventTypeNameGet(eventTypeName, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public EventTypeOut update(final String eventTypeName, final EventTypeUpdate eventTypeUpdate) throws ApiException {
		try {
			return api.updateEventTypeApiV1EventTypeEventTypeNamePut(eventTypeName, eventTypeUpdate, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}

	public void delete(final String eventTypeName) throws ApiException {
		try {
			api.deleteEventTypeApiV1EventTypeEventTypeNameDelete(eventTypeName, null, null);
		} catch (com.svix.internal.ApiException e) {
			throw Utils.wrapInternalApiException(e);
		}
	}
}
