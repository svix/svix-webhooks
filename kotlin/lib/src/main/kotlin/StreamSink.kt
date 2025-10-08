// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.EmptyResponse
import com.svix.kotlin.models.EndpointSecretRotateIn
import com.svix.kotlin.models.ListResponseStreamSinkOut
import com.svix.kotlin.models.Ordering
import com.svix.kotlin.models.SinkSecretOut
import com.svix.kotlin.models.SinkTransformIn
import com.svix.kotlin.models.StreamSinkIn
import com.svix.kotlin.models.StreamSinkOut
import com.svix.kotlin.models.StreamSinkPatch
import okhttp3.Headers

data class StreamSinkListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
)

data class StreamSinkCreateOptions(val idempotencyKey: String? = null)

data class StreamSinkRotateSecretOptions(val idempotencyKey: String? = null)

class StreamSink(private val client: SvixHttpClient) {
    /** List of all the stream's sinks. */
    suspend fun list(
        streamId: String,
        options: StreamSinkListOptions = StreamSinkListOptions(),
    ): ListResponseStreamSinkOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId/sink")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseStreamSinkOut>("GET", url.build())
    }

    /** Creates a new sink. */
    suspend fun create(
        streamId: String,
        streamSinkIn: StreamSinkIn,
        options: StreamSinkCreateOptions = StreamSinkCreateOptions(),
    ): StreamSinkOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId/sink")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<StreamSinkIn, StreamSinkOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = streamSinkIn,
        )
    }

    /** Get a sink by id or uid. */
    suspend fun get(streamId: String, sinkId: String): StreamSinkOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId/sink/$sinkId")
        return client.executeRequest<Any, StreamSinkOut>("GET", url.build())
    }

    /** Update a sink. */
    suspend fun update(
        streamId: String,
        sinkId: String,
        streamSinkIn: StreamSinkIn,
    ): StreamSinkOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId/sink/$sinkId")

        return client.executeRequest<StreamSinkIn, StreamSinkOut>(
            "PUT",
            url.build(),
            reqBody = streamSinkIn,
        )
    }

    /** Delete a sink. */
    suspend fun delete(streamId: String, sinkId: String) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId/sink/$sinkId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Partially update a sink. */
    suspend fun patch(
        streamId: String,
        sinkId: String,
        streamSinkPatch: StreamSinkPatch,
    ): StreamSinkOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId/sink/$sinkId")

        return client.executeRequest<StreamSinkPatch, StreamSinkOut>(
            "PATCH",
            url.build(),
            reqBody = streamSinkPatch,
        )
    }

    /**
     * Get the sink's signing secret (only supported for http sinks)
     *
     * This is used to verify the authenticity of the delivery.
     *
     * For more information please refer to
     * [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    suspend fun getSecret(streamId: String, sinkId: String): SinkSecretOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/stream/$streamId/sink/$sinkId/secret")
        return client.executeRequest<Any, SinkSecretOut>("GET", url.build())
    }

    /** Rotates the signing secret (only supported for http sinks). */
    suspend fun rotateSecret(
        streamId: String,
        sinkId: String,
        endpointSecretRotateIn: EndpointSecretRotateIn,
        options: StreamSinkRotateSecretOptions = StreamSinkRotateSecretOptions(),
    ): EmptyResponse {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/stream/$streamId/sink/$sinkId/secret/rotate")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<EndpointSecretRotateIn, EmptyResponse>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = endpointSecretRotateIn,
        )
    }

    /** Set or unset the transformation code associated with this sink. */
    suspend fun transformationPartialUpdate(
        streamId: String,
        sinkId: String,
        sinkTransformIn: SinkTransformIn,
    ): EmptyResponse {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/stream/$streamId/sink/$sinkId/transformation")

        return client.executeRequest<SinkTransformIn, EmptyResponse>(
            "PATCH",
            url.build(),
            reqBody = sinkTransformIn,
        )
    }
}
