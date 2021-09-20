package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.EventTypeApi
import com.svix.kotlin.internal.infrastructure.ClientException
import com.svix.kotlin.internal.infrastructure.ServerException
import com.svix.kotlin.models.EventTypeIn
import com.svix.kotlin.models.EventTypeOut
import com.svix.kotlin.models.EventTypeUpdate
import com.svix.kotlin.models.ListResponseEventTypeOut

class EventTypeListOptions(var withContent: Boolean = false) : ListOptions() {
    fun withContent(withContent: Boolean) = apply { this.withContent = withContent }

    override fun iterator(iterator: kotlin.String) = apply { super.iterator(iterator) }

    override fun limit(limit: kotlin.Int) = apply { super.limit(limit) }
}

class EventType internal constructor(debugUrl: String) {
    val api = EventTypeApi(debugUrl)

    suspend fun list(options: EventTypeListOptions): ListResponseEventTypeOut {
        try {
            return api.listEventTypesApiV1EventTypeGet(options.iterator, options.limit, options.withContent)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun create(eventTypeIn: EventTypeIn): EventTypeOut {
        try {
            return api.createEventTypeApiV1EventTypePost(eventTypeIn)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun update(eventTypeName: String, eventTypeUpdate: EventTypeUpdate): EventTypeOut {
        try {
            return api.updateEventTypeApiV1EventTypeEventTypeNamePut(eventTypeName, eventTypeUpdate)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun delete(eventTypeName: String) {
        try {
            api.deleteEventTypeApiV1EventTypeEventTypeNameDelete(eventTypeName)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }
}
