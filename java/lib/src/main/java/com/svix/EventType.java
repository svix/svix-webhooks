package com.svix;

import com.svix.generated.ApiException;
import com.svix.generated.api.EventTypeApi;
import com.svix.generated.model.EventTypeIn;
import com.svix.generated.model.EventTypeOut;
import com.svix.generated.model.EventTypeUpdate;
import com.svix.generated.model.ListResponseEventTypeOut;

public final class EventType {
	private final EventTypeApi api;

	public EventType() {
		api = new EventTypeApi();
	}

	public ListResponseEventTypeOut list(final FetchOptions options) throws ApiException {
		return api.listEventTypesApiV1EventTypeGet(options.getIterator(), options.getLimit());
	}

	public EventTypeOut create(final EventTypeIn eventTypeInOut) throws ApiException {
		return api.createEventTypeApiV1EventTypePost(eventTypeInOut);
	}

	public EventTypeOut update(final String eventTypeName, final EventTypeUpdate eventTypeUpdate) throws ApiException {
		return api.updateEventTypeApiV1EventTypeEventTypeNamePut(eventTypeName, eventTypeUpdate);
	}

	public void delete(final String eventTypeName) throws ApiException {
		api.deleteEventTypeApiV1EventTypeEventTypeNameDelete(eventTypeName);
	}
}
