// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.IngestEndpointHeadersIn
import com.svix.kotlin.models.IngestEndpointHeadersOut
import com.svix.kotlin.models.IngestEndpointIn
import com.svix.kotlin.models.IngestEndpointOut
import com.svix.kotlin.models.IngestEndpointSecretIn
import com.svix.kotlin.models.IngestEndpointSecretOut
import com.svix.kotlin.models.IngestEndpointUpdate
import com.svix.kotlin.models.ListResponseIngestEndpointOut
import com.svix.kotlin.models.Ordering
import okhttp3.Headers

data class IngestEndpointListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
)

data class IngestEndpointCreateOptions(val idempotencyKey: String? = null)

data class IngestEndpointRotateSecretOptions(val idempotencyKey: String? = null)

class IngestEndpoint(private val client: SvixHttpClient) {
    /** List ingest endpoints. */
    suspend fun list(
        sourceId: String,
        options: IngestEndpointListOptions = IngestEndpointListOptions(),
    ): ListResponseIngestEndpointOut {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source/$sourceId/endpoint")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseIngestEndpointOut>("GET", url.build())
    }

    /** Create an ingest endpoint. */
    suspend fun create(
        sourceId: String,
        ingestEndpointIn: IngestEndpointIn,
        options: IngestEndpointCreateOptions = IngestEndpointCreateOptions(),
    ): IngestEndpointOut {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source/$sourceId/endpoint")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<IngestEndpointIn, IngestEndpointOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = ingestEndpointIn,
        )
    }

    /** Get an ingest endpoint. */
    suspend fun get(sourceId: String, endpointId: String): IngestEndpointOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/$sourceId/endpoint/$endpointId")
        return client.executeRequest<Any, IngestEndpointOut>("GET", url.build())
    }

    /** Update an ingest endpoint. */
    suspend fun update(
        sourceId: String,
        endpointId: String,
        ingestEndpointUpdate: IngestEndpointUpdate,
    ): IngestEndpointOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/$sourceId/endpoint/$endpointId")

        return client.executeRequest<IngestEndpointUpdate, IngestEndpointOut>(
            "PUT",
            url.build(),
            reqBody = ingestEndpointUpdate,
        )
    }

    /** Delete an ingest endpoint. */
    suspend fun delete(sourceId: String, endpointId: String) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/$sourceId/endpoint/$endpointId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Get the additional headers to be sent with the ingest. */
    suspend fun getHeaders(sourceId: String, endpointId: String): IngestEndpointHeadersOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/$sourceId/endpoint/$endpointId/headers")
        return client.executeRequest<Any, IngestEndpointHeadersOut>("GET", url.build())
    }

    /** Set the additional headers to be sent to the endpoint. */
    suspend fun updateHeaders(
        sourceId: String,
        endpointId: String,
        ingestEndpointHeadersIn: IngestEndpointHeadersIn,
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/$sourceId/endpoint/$endpointId/headers")

        client.executeRequest<IngestEndpointHeadersIn, Boolean>(
            "PUT",
            url.build(),
            reqBody = ingestEndpointHeadersIn,
        )
    }

    /**
     * Get an ingest endpoint's signing secret.
     *
     * This is used to verify the authenticity of the webhook. For more information please refer to
     * [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
     */
    suspend fun getSecret(sourceId: String, endpointId: String): IngestEndpointSecretOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/$sourceId/endpoint/$endpointId/secret")
        return client.executeRequest<Any, IngestEndpointSecretOut>("GET", url.build())
    }

    /**
     * Rotates an ingest endpoint's signing secret.
     *
     * The previous secret will remain valid for the next 24 hours.
     */
    suspend fun rotateSecret(
        sourceId: String,
        endpointId: String,
        ingestEndpointSecretIn: IngestEndpointSecretIn,
        options: IngestEndpointRotateSecretOptions = IngestEndpointRotateSecretOptions(),
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/$sourceId/endpoint/$endpointId/secret/rotate")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        client.executeRequest<IngestEndpointSecretIn, Boolean>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = ingestEndpointSecretIn,
        )
    }
}
