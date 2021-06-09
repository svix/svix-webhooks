package com.svix;

import com.svix.generated.ApiException;
import com.svix.generated.api.EventTypeApi;
import com.svix.model.EventTypeInOut;
import com.svix.model.EventTypeUpdate;
import com.svix.model.ListResponseEventTypeInOut;

public final class EventType {
	private final EventTypeApi api;

	public EventType() {
		api = new EventTypeApi();
	}

	public ListResponseEventTypeInOut list(final FetchOptions options) throws ApiException {
		return api.listEventTypesApiV1EventTypeGet(options.getIterator(), options.getLimit());
	}

	public EventTypeInOut create(final EventTypeInOut eventTypeInOut) throws ApiException {
		return api.createEventTypeApiV1EventTypePost(eventTypeInOut);
	}

	public EventTypeInOut update(final String eventTypeName, final EventTypeUpdate eventTypeUpdate) throws ApiException {
		return api.updateEventTypeApiV1EventTypeEventTypeNamePut(eventTypeName, eventTypeUpdate);
	}

	public void delete(final String eventTypeName) throws ApiException {
		api.deleteEventTypeApiV1EventTypeEventTypeNameDelete(eventTypeName);
	}
}
