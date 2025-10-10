// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.ListResponseStreamEventTypeOut
import com.svix.kotlin.models.Ordering
import com.svix.kotlin.models.StreamEventTypeIn
import com.svix.kotlin.models.StreamEventTypeOut
import com.svix.kotlin.models.StreamEventTypePatch
import okhttp3.Headers

data class StreamingEventTypeListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
    /** Include archived (deleted but not expunged) items in the response. */
    val includeArchived: Boolean? = null,
)

data class StreamingEventTypeCreateOptions(val idempotencyKey: String? = null)

data class StreamingEventTypeDeleteOptions(
    /**
     * By default, event types are archived when "deleted". With this flag, they are deleted
     * entirely.
     */
    val expunge: Boolean? = null
)

class StreamingEventType(private val client: SvixHttpClient) {
    /** List of all the organization's event types for streaming. */
    suspend fun list(
        options: StreamingEventTypeListOptions = StreamingEventTypeListOptions()
    ): ListResponseStreamEventTypeOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/event-type")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        options.includeArchived?.let {
            url.addQueryParameter("include_archived", serializeQueryParam(it))
        }
        return client.executeRequest<Any, ListResponseStreamEventTypeOut>("GET", url.build())
    }

    /** Create an event type for Streams. */
    suspend fun create(
        streamEventTypeIn: StreamEventTypeIn,
        options: StreamingEventTypeCreateOptions = StreamingEventTypeCreateOptions(),
    ): StreamEventTypeOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/event-type")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<StreamEventTypeIn, StreamEventTypeOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = streamEventTypeIn,
        )
    }

    /** Get an event type. */
    suspend fun get(name: String): StreamEventTypeOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/event-type/$name")
        return client.executeRequest<Any, StreamEventTypeOut>("GET", url.build())
    }

    /** Update or create a event type for Streams. */
    suspend fun update(name: String, streamEventTypeIn: StreamEventTypeIn): StreamEventTypeOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/event-type/$name")

        return client.executeRequest<StreamEventTypeIn, StreamEventTypeOut>(
            "PUT",
            url.build(),
            reqBody = streamEventTypeIn,
        )
    }

    /** Delete an event type. */
    suspend fun delete(
        name: String,
        options: StreamingEventTypeDeleteOptions = StreamingEventTypeDeleteOptions(),
    ) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/event-type/$name")
        options.expunge?.let { url.addQueryParameter("expunge", serializeQueryParam(it)) }
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Patch an event type for Streams. */
    suspend fun patch(
        name: String,
        streamEventTypePatch: StreamEventTypePatch,
    ): StreamEventTypeOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/event-type/$name")

        return client.executeRequest<StreamEventTypePatch, StreamEventTypeOut>(
            "PATCH",
            url.build(),
            reqBody = streamEventTypePatch,
        )
    }
}
