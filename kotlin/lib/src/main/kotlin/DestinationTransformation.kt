// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.DestinationTransformIn
import com.svix.kotlin.models.DestinationTransformationOut
import com.svix.kotlin.models.EmptyResponse

class DestinationTransformation(private val client: SvixHttpClient) {
    /** Get the transformation code associated with this destination. */
    suspend fun get(appId: String, destinationId: String): DestinationTransformationOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/destination/$destinationId/transformation")
        return client.executeRequest<Any, DestinationTransformationOut>("GET", url.build())
    }

    /** Set or unset the transformation code associated with this destination. */
    suspend fun patch(
        appId: String,
        destinationId: String,
        destinationTransformIn: DestinationTransformIn,
    ): EmptyResponse {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/destination/$destinationId/transformation")

        return client.executeRequest<DestinationTransformIn, EmptyResponse>(
            "PATCH",
            url.build(),
            reqBody = destinationTransformIn,
        )
    }
}
