// this file is @generated
package com.svix.kotlin.internal

import com.svix.kotlin.SvixHttpClient
import com.svix.kotlin.models.EndpointOut
import com.svix.kotlin.models.SubscribeIn

class EndpointAutoConfig(private val client: SvixHttpClient) {
    /** Update an auto-config endpoint by providing endpoint details. */
    suspend fun update(appId: String, endpointId: String, subscribeIn: SubscribeIn): EndpointOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/endpoint/$endpointId/auto-config")

        return client.executeRequest<SubscribeIn, EndpointOut>(
            "PUT",
            url.build(),
            reqBody = subscribeIn,
        )
    }
}
