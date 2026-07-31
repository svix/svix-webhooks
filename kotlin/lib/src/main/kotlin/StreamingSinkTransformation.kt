// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.EmptyResponse
import com.svix.kotlin.models.SinkTransformIn
import com.svix.kotlin.models.SinkTransformationOut

class StreamingSinkTransformation(private val client: SvixHttpClient) {
    /** Get the transformation code associated with this sink. */
    suspend fun get(streamId: String, sinkId: String): SinkTransformationOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/stream/$streamId/sink/$sinkId/transformation")
        return client.executeRequest<Any, SinkTransformationOut>("GET", url.build())
    }

    /** Set or unset the transformation code associated with this sink. */
    suspend fun patch(
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
