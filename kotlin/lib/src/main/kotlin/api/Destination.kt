// this file is @generated
package com.svix.kotlin.api

import com.svix.kotlin.SvixHttpClient
import com.svix.kotlin.models.DestinationIn
import com.svix.kotlin.models.DestinationOut
import com.svix.kotlin.models.DestinationPatch
import com.svix.kotlin.models.ListResponseDestinationOut
import com.svix.kotlin.models.Ordering
import com.svix.kotlin.serializeQueryParam
import okhttp3.Headers

data class DestinationListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
)

data class DestinationCreateOptions(val idempotencyKey: String? = null)

class Destination(private val client: SvixHttpClient) {
    val transformation: DestinationTransformation = DestinationTransformation(client)

    /** List of all the application's destinations. */
    suspend fun list(
        appId: String,
        options: DestinationListOptions = DestinationListOptions(),
    ): ListResponseDestinationOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/destination")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseDestinationOut>("GET", url.build())
    }

    /** Creates a new destination. */
    suspend fun create(
        appId: String,
        destinationIn: DestinationIn,
        options: DestinationCreateOptions = DestinationCreateOptions(),
    ): DestinationOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/app/$appId/destination")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<DestinationIn, DestinationOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = destinationIn,
        )
    }

    /** Get a destination by id or uid. */
    suspend fun get(appId: String, destinationId: String): DestinationOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/destination/$destinationId")
        return client.executeRequest<Any, DestinationOut>("GET", url.build())
    }

    /** Create or update a destination. */
    suspend fun upsert(
        appId: String,
        destinationId: String,
        destinationIn: DestinationIn,
    ): DestinationOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/destination/$destinationId")

        return client.executeRequest<DestinationIn, DestinationOut>(
            "PUT",
            url.build(),
            reqBody = destinationIn,
        )
    }

    /** Delete a destination. */
    suspend fun delete(appId: String, destinationId: String) {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/destination/$destinationId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Partially update a destination. */
    suspend fun patch(
        appId: String,
        destinationId: String,
        destinationPatch: DestinationPatch,
    ): DestinationOut {
        val url =
            client.newUrlBuilder().encodedPath("/api/v1/app/$appId/destination/$destinationId")

        return client.executeRequest<DestinationPatch, DestinationOut>(
            "PATCH",
            url.build(),
            reqBody = destinationPatch,
        )
    }
}
