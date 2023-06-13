package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.EventTypeApi
import com.svix.kotlin.models.EventTypeIn
import com.svix.kotlin.models.EventTypeOut
import com.svix.kotlin.models.EventTypeUpdate
import com.svix.kotlin.models.ListResponseEventTypeOut

class EventType internal constructor(token: String, options: SvixOptions) {
    val api = EventTypeApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    suspend fun list(options: EventTypeListOptions = EventTypeListOptions()): ListResponseEventTypeOut {
        try {
            return api.v1EventTypeList(options.limit, options.iterator, null, options.includeAchived, options.withContent)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(eventTypeIn: EventTypeIn, options: PostOptions = PostOptions()): EventTypeOut {
        try {
            return api.v1EventTypeCreate(eventTypeIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun get(eventTypeName: String): EventTypeOut {
        try {
            return api.v1EventTypeGet(eventTypeName)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun update(eventTypeName: String, eventTypeUpdate: EventTypeUpdate): EventTypeOut {
        try {
            return api.v1EventTypeUpdate(eventTypeName, eventTypeUpdate)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun delete(eventTypeName: String) {
        try {
            api.v1EventTypeDelete(eventTypeName, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
