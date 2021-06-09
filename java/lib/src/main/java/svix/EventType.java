package com.svix;

import com.svix.generated.ApiException;
import com.svix.generated.api.EventTypeApi;
import com.svix.generated.model.EventTypeInOut;
import com.svix.generated.model.EventTypeUpdate;
import com.svix.generated.model.ListResponseEventTypeInOut;

public class EventType {
	private final EventTypeApi api;

	public EventType() {
		api = new EventTypeApi();
	}

	public ListResponseEventTypeInOut list(final String iterator, final Integer limit) throws ApiException {
		return api.listEventTypesApiV1EventTypeGet(iterator, limit);
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
