// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.CreateStreamEventsIn
import com.svix.kotlin.models.CreateStreamEventsOut
import com.svix.kotlin.models.EventStreamOut
import kotlinx.datetime.Instant
import okhttp3.Headers

data class StreamEventsCreateOptions(val idempotencyKey: String? = null)

data class StreamEventsGetOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    val after: Instant? = null,
)

class StreamEvents(private val client: SvixHttpClient) {
    /** Creates events on the Stream. */
    suspend fun create(
        streamId: String,
        createStreamEventsIn: CreateStreamEventsIn,
        options: StreamEventsCreateOptions = StreamEventsCreateOptions(),
    ): CreateStreamEventsOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId/events")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<CreateStreamEventsIn, CreateStreamEventsOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = createStreamEventsIn,
        )
    }

    /**
     * Iterate over a stream of events.
     *
     * The sink must be of type `poller` to use the poller endpoint.
     */
    suspend fun get(
        streamId: String,
        sinkId: String,
        options: StreamEventsGetOptions = StreamEventsGetOptions(),
    ): EventStreamOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId/sink/$sinkId/events")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.after?.let { url.addQueryParameter("after", serializeQueryParam(it)) }
        return client.executeRequest<Any, EventStreamOut>("GET", url.build())
    }
}
