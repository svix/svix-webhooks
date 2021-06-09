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

	public ListResponseEventTypeInOut list(String iterator, Integer limit) throws ApiException {
		return api.listEventTypesApiV1EventTypeGet(iterator, limit);
	}

	public EventTypeInOut create(EventTypeInOut eventTypeInOut) throws ApiException {
		return api.createEventTypeApiV1EventTypePost(eventTypeInOut);
	}

	public EventTypeInOut update(String eventTypeName, EventTypeUpdate eventTypeUpdate) throws ApiException {
		return api.updateEventTypeApiV1EventTypeEventTypeNamePut(eventTypeName, eventTypeUpdate);
	}

	public void delete(String eventTypeName) throws ApiException {
		api.deleteEventTypeApiV1EventTypeEventTypeNameDelete(eventTypeName);
	}
}
