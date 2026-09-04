// this file is @generated
package com.svix.kotlin.internal

import com.svix.kotlin.SvixHttpClient
import com.svix.kotlin.models.DestinationIn
import com.svix.kotlin.models.DestinationOut

class DestinationAutoconfig(private val client: SvixHttpClient) {
    /** Create or update the destination for an AutoConfig subscription. */
    suspend fun subscribe(
        appId: String,
        autoconfigId: String,
        destinationIn: DestinationIn,
    ): DestinationOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/autoconfig/$autoconfigId/destination")

        return client.executeRequest<DestinationIn, DestinationOut>(
            "PUT",
            url.build(),
            reqBody = destinationIn,
        )
    }
}
