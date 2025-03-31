// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.ApiTokenExpireIn
import com.svix.kotlin.models.ApiTokenIn
import com.svix.kotlin.models.ApiTokenOut
import com.svix.kotlin.models.ListResponseApiTokenCensoredOut
import com.svix.kotlin.models.Ordering
import okhttp3.Headers

data class ManagementAuthenticationListApiTokensOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
)

data class ManagementAuthenticationCreateApiTokenOptions(val idempotencyKey: String? = null)

data class ManagementAuthenticationExpireApiTokenOptions(val idempotencyKey: String? = null)

class ManagementAuthentication(private val client: SvixHttpClient) {
    /** List all API Tokens. */
    suspend fun listApiTokens(
        options: ManagementAuthenticationListApiTokensOptions =
            ManagementAuthenticationListApiTokensOptions()
    ): ListResponseApiTokenCensoredOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/management/authentication/api-token")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseApiTokenCensoredOut>("GET", url.build())
    }

    /** Create a new API Token. */
    suspend fun createApiToken(
        apiTokenIn: ApiTokenIn,
        options: ManagementAuthenticationCreateApiTokenOptions =
            ManagementAuthenticationCreateApiTokenOptions(),
    ): ApiTokenOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/management/authentication/api-token")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<ApiTokenIn, ApiTokenOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = apiTokenIn,
        )
    }

    /** Expire the selected API Token. */
    suspend fun expireApiToken(
        keyId: String,
        apiTokenExpireIn: ApiTokenExpireIn,
        options: ManagementAuthenticationExpireApiTokenOptions =
            ManagementAuthenticationExpireApiTokenOptions(),
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/management/authentication/api-token/$keyId/expire")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        client.executeRequest<ApiTokenExpireIn, Boolean>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = apiTokenExpireIn,
        )
    }
}
