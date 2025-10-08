// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.EndpointHeadersOut
import com.svix.kotlin.models.HttpSinkHeadersPatchIn
import com.svix.kotlin.models.SinkTransformationOut

class Stream(private val client: SvixHttpClient) {
    val eventType: StreamEventType = StreamEventType(client)

    val events: StreamEvents = StreamEvents(client)

    val sink: StreamSink = StreamSink(client)

    val stream: StreamStream = StreamStream(client)

    /** Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks. */
    suspend fun sinkHeadersGet(streamId: String, sinkId: String): EndpointHeadersOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId/sink/$sinkId/headers")
        return client.executeRequest<Any, EndpointHeadersOut>("GET", url.build())
    }

    /** Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks. */
    suspend fun sinkHeadersPatch(
        streamId: String,
        sinkId: String,
        httpSinkHeadersPatchIn: HttpSinkHeadersPatchIn,
    ): EndpointHeadersOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId/sink/$sinkId/headers")

        return client.executeRequest<HttpSinkHeadersPatchIn, EndpointHeadersOut>(
            "PATCH",
            url.build(),
            reqBody = httpSinkHeadersPatchIn,
        )
    }

    /** Get the transformation code associated with this sink. */
    suspend fun sinkTransformationGet(streamId: String, sinkId: String): SinkTransformationOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/stream/$streamId/sink/$sinkId/transformation")
        return client.executeRequest<Any, SinkTransformationOut>("GET", url.build())
    }
}
