// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.AutoConfigOut
import com.svix.kotlin.models.CreateAutoConfigSubscriptionIn
import com.svix.kotlin.models.RotateSubscriptionIn2
import okhttp3.Headers

data class AutoconfigSubscriptionCreateOptions(val idempotencyKey: String? = null)

data class AutoconfigSubscriptionRotateOptions(val idempotencyKey: String? = null)

class AutoconfigSubscription(private val client: SvixHttpClient) {
    /** Create an AutoConfig subscription. */
    suspend fun create(
        appId: String,
        createAutoConfigSubscriptionIn: CreateAutoConfigSubscriptionIn,
        options: AutoconfigSubscriptionCreateOptions = AutoconfigSubscriptionCreateOptions(),
    ): AutoConfigOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/autoconfig")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<CreateAutoConfigSubscriptionIn, AutoConfigOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = createAutoConfigSubscriptionIn,
        )
    }

    /** Delete an AutoConfig subscription. This also invalidates its auth token. */
    suspend fun delete(appId: String, autoconfigId: String) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/autoconfig/$autoconfigId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Rotate the auth token and signing secret for an AutoConfig subscription. */
    suspend fun rotate(
        appId: String,
        autoconfigId: String,
        rotateSubscriptionIn2: RotateSubscriptionIn2,
        options: AutoconfigSubscriptionRotateOptions = AutoconfigSubscriptionRotateOptions(),
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
