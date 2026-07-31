// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.EndpointTransformationOut
import com.svix.kotlin.models.EndpointTransformationPatch

class EndpointTransformation(private val client: SvixHttpClient) {
    /** Get the transformation code associated with this endpoint. */
    suspend fun get(appId: String, endpointId: String): EndpointTransformationOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/endpoint/$endpointId/transformation")
        return client.executeRequest<Any, EndpointTransformationOut>("GET", url.build())
    }

    /** Set or unset the transformation code associated with this endpoint. */
    suspend fun patch(
        appId: String,
        endpointId: String,
        endpointTransformationPatch: EndpointTransformationPatch,
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/endpoint/$endpointId/transformation")

        client.executeRequest<EndpointTransformationPatch, Boolean>(
            "PATCH",
            url.build(),
            reqBody = endpointTransformationPatch,
        )
    }
}
