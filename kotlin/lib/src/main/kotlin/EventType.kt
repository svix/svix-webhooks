package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.EventTypeApi
import com.svix.kotlin.models.EventTypeImportOpenApiIn
import com.svix.kotlin.models.EventTypeImportOpenApiOut
import com.svix.kotlin.models.EventTypeIn
import com.svix.kotlin.models.EventTypeOut
import com.svix.kotlin.models.EventTypePatch
import com.svix.kotlin.models.EventTypeUpdate
import com.svix.kotlin.models.ListResponseEventTypeOut
import com.svix.kotlin.models.Ordering

class EventTypeListOptions {
    var limit: Int? = null
    var iterator: String? = null
    var order: Ordering? = null
    var includeArchived: Boolean? = null
    var withContent: Boolean? = null

    /** Limit the number of returned items */
    fun limit(limit: Int) = apply { this.limit = limit }

    /** The iterator returned from a prior invocation */
    fun iterator(iterator: String) = apply { this.iterator = iterator }

    /** The sorting order of the returned items */
    fun order(order: Ordering) = apply { this.order = order }

    /** When `true` archived (deleted but not expunged) items are included in the response. */
    fun includeArchived(includeArchived: Boolean) = apply { this.includeArchived = includeArchived }

    @Deprecated("Use the new includeArchived() method")
    fun includeAchived(includeArchived: Boolean) = apply { this.includeArchived = includeArchived }

    /** When `true` the full item (including the schema) is included in the response. */
    fun withContent(withContent: Boolean) = apply { this.withContent = withContent }
}

class EventType internal constructor(token: String, options: SvixOptions) {
    val api = EventTypeApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    suspend fun list(
        options: EventTypeListOptions = EventTypeListOptions()
    ): ListResponseEventTypeOut {
        try {
            return api.v1EventTypeList(
                options.limit,
                options.iterator,
                null,
                options.includeArchived,
                options.withContent,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    suspend fun create(
        eventTypeIn: EventTypeIn,
        options: PostOptions = PostOptions(),
    ): EventTypeOut {
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

    suspend fun patch(eventTypeName: String, eventTypePatch: EventTypePatch): EventTypeOut {
        try {
            return api.v1EventTypePatch(eventTypeName, eventTypePatch)
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

    suspend fun importOpenApi(
        eventTypeImportOpenApiIn: EventTypeImportOpenApiIn,
        options: PostOptions = PostOptions(),
    ): EventTypeImportOpenApiOut {
        try {
            return api.v1EventTypeImportOpenapi(eventTypeImportOpenApiIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
