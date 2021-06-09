package com.svix;

import com.svix.ApiException;
import com.svix.api.EventTypeApi;
import com.svix.model.EventTypeIn;
import com.svix.model.EventTypeOut;
import com.svix.model.EventTypeUpdate;
import com.svix.model.ListResponseEventTypeOut;

public final class EventType {
	private final EventTypeApi api;

	public EventType() {
		api = new EventTypeApi();
	}

	public ListResponseEventTypeOut list(final FetchOptions options) throws ApiException {
		return api.listEventTypesApiV1EventTypeGet(options.getIterator(), options.getLimit());
	}

	public EventTypeOut create(final EventTypeIn eventTypeIn) throws ApiException {
		return api.createEventTypeApiV1EventTypePost(eventTypeIn);
	}

	public EventTypeOut update(final String eventTypeName, final EventTypeUpdate eventTypeUpdate) throws ApiException {
		return api.updateEventTypeApiV1EventTypeEventTypeNamePut(eventTypeName, eventTypeUpdate);
	}

	public void delete(final String eventTypeName) throws ApiException {
		api.deleteEventTypeApiV1EventTypeEventTypeNameDelete(eventTypeName);
	}
}
