// this file is @generated
package com.svix.kotlin.internal

import com.svix.kotlin.SvixHttpClient
import com.svix.kotlin.models.AutoConfigSubscriptionOut

class Autoconfig(private val client: SvixHttpClient) {
    val destination: AutoconfigDestination = AutoconfigDestination(client)

    val endpoint: AutoconfigEndpoint = AutoconfigEndpoint(client)

    /** Get an AutoConfig subscription, including the bound endpoint or destination if any. */
    suspend fun get(appId: String, autoconfigId: String): AutoConfigSubscriptionOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/autoconfig/$autoconfigId")
        return client.executeRequest<Any, AutoConfigSubscriptionOut>("GET", url.build())
    }
}
