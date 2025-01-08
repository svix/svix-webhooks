// this file is @generated (with minor manual changes)
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
    private val api = EventTypeApi(options.serverUrl)

    init {
        api.accessToken = token
        api.userAgent = options.getUA()
        options.initialRetryDelayMillis?.let { api.initialRetryDelayMillis = it }
        options.numRetries?.let { api.numRetries = it }
    }

    /** Return the list of event types. */
    suspend fun list(
        options: EventTypeListOptions = EventTypeListOptions()
    ): ListResponseEventTypeOut {
        try {
            return api.v1EventTypeList(
                options.limit,
                options.iterator,
                options.order,
                options.includeArchived,
                options.withContent,
            )
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * Create new or unarchive existing event type.
     *
     * Unarchiving an event type will allow endpoints to filter on it and messages to be sent with
     * it. Endpoints filtering on the event type before archival will continue to filter on it. This
     * operation does not preserve the description and schemas.
     */
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

    /**
     * Given an OpenAPI spec, create new or update existing event types. If an existing `archived`
     * event type is updated, it will be unarchived.
     *
     * The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
     * top-level.
     */
    suspend fun importOpenapi(
        eventTypeImportOpenApiIn: EventTypeImportOpenApiIn,
        options: PostOptions = PostOptions(),
    ): EventTypeImportOpenApiOut {
        try {
            return api.v1EventTypeImportOpenapi(eventTypeImportOpenApiIn, options.idempotencyKey)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Get an event type. */
    suspend fun get(eventTypeName: String): EventTypeOut {
        try {
            return api.v1EventTypeGet(eventTypeName)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Update an event type. */
    suspend fun update(eventTypeName: String, eventTypeUpdate: EventTypeUpdate): EventTypeOut {
        try {
            return api.v1EventTypeUpdate(eventTypeName, eventTypeUpdate)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /**
     * Archive an event type.
     *
     * Endpoints already configured to filter on an event type will continue to do so after
     * archival. However, new messages can not be sent with it and endpoints can not filter on it.
     * An event type can be unarchived with the
     * [create operation](#operation/create_event_type_api_v1_event_type__post).
     */
    suspend fun delete(eventTypeName: String) {
        try {
            api.v1EventTypeDelete(eventTypeName, null)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }

    /** Partially update an event type. */
    suspend fun patch(eventTypeName: String, eventTypePatch: EventTypePatch): EventTypeOut {
        try {
            return api.v1EventTypePatch(eventTypeName, eventTypePatch)
        } catch (e: Exception) {
            throw ApiException.wrap(e)
        }
    }
}
