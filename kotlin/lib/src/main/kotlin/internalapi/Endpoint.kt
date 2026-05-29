// this file is @generated
package com.svix.kotlin.internal

import com.svix.kotlin.SvixHttpClient
import com.svix.kotlin.models.EndpointTransformationIn

class Endpoint(private val client: SvixHttpClient) {
    val autoConfig: EndpointAutoConfig = EndpointAutoConfig(client)

    /**
     * This operation was renamed to `set-transformation`.
     *
     * @deprecated
     */
    @Deprecated("")
    suspend fun transformationPartialUpdate(
        appId: String,
        endpointId: String,
        endpointTransformationIn: EndpointTransformationIn,
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/endpoint/$endpointId/transformation")

        client.executeRequest<EndpointTransformationIn, Boolean>(
            "PATCH",
            url.build(),
            reqBody = endpointTransformationIn,
        )
    }
}
