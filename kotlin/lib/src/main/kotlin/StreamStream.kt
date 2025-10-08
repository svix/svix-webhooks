// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.ListResponseStreamOut
import com.svix.kotlin.models.Ordering
import com.svix.kotlin.models.StreamIn
import com.svix.kotlin.models.StreamOut
import com.svix.kotlin.models.StreamPatch
import okhttp3.Headers

data class StreamStreamListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
)

data class StreamStreamCreateOptions(val idempotencyKey: String? = null)

class StreamStream(private val client: SvixHttpClient) {
    /** List of all the organization's streams. */
    suspend fun list(
        options: StreamStreamListOptions = StreamStreamListOptions()
    ): ListResponseStreamOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseStreamOut>("GET", url.build())
    }

    /** Creates a new stream. */
    suspend fun create(
        streamIn: StreamIn,
        options: StreamStreamCreateOptions = StreamStreamCreateOptions(),
    ): StreamOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<StreamIn, StreamOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = streamIn,
        )
    }

    /** Get a stream by id or uid. */
    suspend fun get(streamId: String): StreamOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId")
        return client.executeRequest<Any, StreamOut>("GET", url.build())
    }

    /** Update a stream. */
    suspend fun update(streamId: String, streamIn: StreamIn): StreamOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId")

        return client.executeRequest<StreamIn, StreamOut>("PUT", url.build(), reqBody = streamIn)
    }

    /** Delete a stream. */
    suspend fun delete(streamId: String) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Partially update a stream. */
    suspend fun patch(streamId: String, streamPatch: StreamPatch): StreamOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId")

        return client.executeRequest<StreamPatch, StreamOut>(
            "PATCH",
            url.build(),
            reqBody = streamPatch,
        )
    }
}
