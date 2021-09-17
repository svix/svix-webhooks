package com.svix.kotlin

import com.svix.kotlin.internal.apis.EventTypeApi
import com.svix.kotlin.internal.infrastructure.ServerException
import com.svix.kotlin.internal.models.EventTypeIn
import com.svix.kotlin.internal.models.EventTypeOut
import com.svix.kotlin.internal.models.EventTypeUpdate
import com.svix.kotlin.internal.models.ListResponseEventTypeOut

class EventType() {
	val api = EventTypeApi()

	suspend fun list(options: EventTypeListOptions): ListResponseEventTypeOut {
		try {
			return api.listEventTypesApiV1EventTypeGet(options.iterator, options.limit, options.withContent)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun create(eventTypeIn: EventTypeIn): EventTypeOut {
		try {
			return api.createEventTypeApiV1EventTypePost(eventTypeIn)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun update(eventTypeName: String, eventTypeUpdate: EventTypeUpdate): EventTypeOut {
		try {
			return api.updateEventTypeApiV1EventTypeEventTypeNamePut(eventTypeName, eventTypeUpdate)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}

	suspend fun delete(eventTypeName: String) {
		try {
			api.deleteEventTypeApiV1EventTypeEventTypeNameDelete(eventTypeName)
		} catch (ex: Exception) {
			// TODO: Wrap expeption with new expection
			throw ServerException(ex.message)
		}
	}
}
