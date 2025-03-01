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
        options: IngestEndpointListOptions = IngestEndpointListOptions()
    ): ListResponseIngestEndpointOut {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source/{source_id}/endpoint")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseIngestEndpointOut>("GET", url.build())
    }

    /** Create an ingest endpoint. */
    suspend fun create(
        ingestEndpointIn: IngestEndpointIn,
        options: IngestEndpointCreateOptions = IngestEndpointCreateOptions(),
    ): IngestEndpointOut {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source/{source_id}/endpoint")
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
    suspend fun get(endpointId: String): IngestEndpointOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/{source_id}/endpoint/$endpointId")
        return client.executeRequest<Any, IngestEndpointOut>("GET", url.build())
    }

    /** Update an ingest endpoint. */
    suspend fun update(
        endpointId: String,
        ingestEndpointUpdate: IngestEndpointUpdate,
    ): IngestEndpointOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/{source_id}/endpoint/$endpointId")

        return client.executeRequest<IngestEndpointUpdate, IngestEndpointOut>(
            "PUT",
            url.build(),
            reqBody = ingestEndpointUpdate,
        )
    }

    /** Delete an ingest endpoint. */
    suspend fun delete(endpointId: String) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/{source_id}/endpoint/$endpointId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Get the additional headers to be sent with the ingest. */
    suspend fun getHeaders(endpointId: String): IngestEndpointHeadersOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/{source_id}/endpoint/$endpointId/headers")
        return client.executeRequest<Any, IngestEndpointHeadersOut>("GET", url.build())
    }

    /** Set the additional headers to be sent to the endpoint. */
    suspend fun updateHeaders(
        endpointId: String,
        ingestEndpointHeadersIn: IngestEndpointHeadersIn,
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/{source_id}/endpoint/$endpointId/headers")

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
    suspend fun getSecret(endpointId: String): IngestEndpointSecretOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/{source_id}/endpoint/$endpointId/secret")
        return client.executeRequest<Any, IngestEndpointSecretOut>("GET", url.build())
    }

    /**
     * Rotates an ingest endpoint's signing secret.
     *
     * The previous secret will remain valid for the next 24 hours.
     */
    suspend fun rotateSecret(
        endpointId: String,
        ingestEndpointSecretIn: IngestEndpointSecretIn,
        options: IngestEndpointRotateSecretOptions = IngestEndpointRotateSecretOptions(),
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/{source_id}/endpoint/$endpointId/secret/rotate")
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
