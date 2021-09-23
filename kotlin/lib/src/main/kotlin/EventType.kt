package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.EventTypeApi
import com.svix.kotlin.models.EventTypeIn
import com.svix.kotlin.models.EventTypeOut
import com.svix.kotlin.models.EventTypeUpdate
import com.svix.kotlin.models.ListResponseEventTypeOut

class EventType internal constructor(token: String, options: SvixOptions) {
    val api = EventTypeApi(options.debugUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
    }

    suspend fun list(options: EventTypeListOptions = EventTypeListOptions()): ListResponseEventTypeOut {
        try {
            return api.listEventTypesApiV1EventTypeGet(options.iterator, options.limit, options.withContent)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(eventTypeIn: EventTypeIn): EventTypeOut {
        try {
            return api.createEventTypeApiV1EventTypePost(eventTypeIn)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(eventTypeName: String, eventTypeUpdate: EventTypeUpdate): EventTypeOut {
        try {
            return api.updateEventTypeApiV1EventTypeEventTypeNamePut(eventTypeName, eventTypeUpdate)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(eventTypeName: String) {
        try {
            api.deleteEventTypeApiV1EventTypeEventTypeNameDelete(eventTypeName)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
