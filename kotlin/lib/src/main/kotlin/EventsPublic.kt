// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.PollingEndpointConsumerSeekIn
import com.svix.kotlin.models.PollingEndpointConsumerSeekOut
import com.svix.kotlin.models.PollingEndpointOut
import okhttp3.Headers

data class EventsPublicConsumerOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** Filters messages sent with this event type (optional). */
    val eventType: String? = null,
    /** Filters messages sent with this channel (optional). */
    val channel: String? = null,
)

data class EventsPublicConsumerSeekOptions(val idempotencyKey: String? = null)

class EventsPublic(private val client: SvixHttpClient) {
    /**
     * Reads the stream of created messages for an application, filtered on the Sink's event types
     * and Channels, using server-managed iterator tracking.
     */
    suspend fun consumer(
        appId: String,
        sinkId: String,
        consumerId: String,
        options: EventsPublicConsumerOptions = EventsPublicConsumerOptions(),
    ): PollingEndpointOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/poller/$sinkId/consumer/$consumerId")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.eventType?.let { url.addQueryParameter("event_type", it) }
        options.channel?.let { url.addQueryParameter("channel", it) }
        return client.executeRequest<Any, PollingEndpointOut>("GET", url.build())
    }

    /** Sets the starting offset for the consumer of a polling endpoint. */
    suspend fun consumerSeek(
        appId: String,
        sinkId: String,
        consumerId: String,
        pollingEndpointConsumerSeekIn: PollingEndpointConsumerSeekIn,
        options: EventsPublicConsumerSeekOptions = EventsPublicConsumerSeekOptions(),
    ): PollingEndpointConsumerSeekOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/poller/$sinkId/consumer/$consumerId/seek")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<PollingEndpointConsumerSeekIn, PollingEndpointConsumerSeekOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = pollingEndpointConsumerSeekIn,
        )
    }
}
