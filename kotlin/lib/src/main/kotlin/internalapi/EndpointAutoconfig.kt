// this file is @generated
package com.svix.kotlin.internal

import com.svix.kotlin.SvixHttpClient
import com.svix.kotlin.models.EndpointIn
import com.svix.kotlin.models.EndpointOut

class EndpointAutoconfig(private val client: SvixHttpClient) {
    /** Create or update the HTTP endpoint for an AutoConfig subscription. */
    suspend fun subscribe(
        appId: String,
        autoconfigId: String,
        endpointIn: EndpointIn,
    ): EndpointOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/autoconfig/$autoconfigId/endpoint")

        return client.executeRequest<EndpointIn, EndpointOut>(
            "PUT",
            url.build(),
            reqBody = endpointIn,
        )
    }
}
