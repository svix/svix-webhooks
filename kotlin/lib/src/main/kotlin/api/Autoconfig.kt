// this file is @generated
package com.svix.kotlin.api

import com.svix.kotlin.SvixHttpClient
import com.svix.kotlin.models.AutoConfigOut
import com.svix.kotlin.models.RotateSubscriptionIn2
import okhttp3.Headers

data class AutoconfigCreateOptions(val idempotencyKey: String? = null)

data class AutoconfigRotateOptions(val idempotencyKey: String? = null)

class Autoconfig(private val client: SvixHttpClient) {
    /** Create an AutoConfig subscription. */
    suspend fun create(
        appId: String,
        options: AutoconfigCreateOptions = AutoconfigCreateOptions(),
    ): AutoConfigOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/autoconfig")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }
        return client.executeRequest<Any, AutoConfigOut>(
            "POST",
            url.build(),
            headers = headers.build(),
        )
    }

    /** Rotate the auth token and signing secret for an AutoConfig subscription. */
    suspend fun rotate(
        appId: String,
        autoconfigId: String,
        rotateSubscriptionIn2: RotateSubscriptionIn2,
        options: AutoconfigRotateOptions = AutoconfigRotateOptions(),
    ): AutoConfigOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/autoconfig/$autoconfigId/rotate")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<RotateSubscriptionIn2, AutoConfigOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = rotateSubscriptionIn2,
        )
    }
}
