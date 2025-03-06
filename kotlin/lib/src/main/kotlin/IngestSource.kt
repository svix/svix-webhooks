// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.IngestSourceIn
import com.svix.kotlin.models.IngestSourceOut
import com.svix.kotlin.models.ListResponseIngestSourceOut
import com.svix.kotlin.models.Ordering
import com.svix.kotlin.models.RotateTokenOut
import okhttp3.Headers

data class IngestSourceListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
)

data class IngestSourceCreateOptions(val idempotencyKey: String? = null)

data class IngestSourceRotateTokenOptions(val idempotencyKey: String? = null)

class IngestSource(private val client: SvixHttpClient) {
    /** List of all the organization's Ingest Sources. */
    suspend fun list(
        options: IngestSourceListOptions = IngestSourceListOptions()
    ): ListResponseIngestSourceOut {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseIngestSourceOut>("GET", url.build())
    }

    /** Create Ingest Source. */
    suspend fun create(
        ingestSourceIn: IngestSourceIn,
        options: IngestSourceCreateOptions = IngestSourceCreateOptions(),
    ): IngestSourceOut {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<IngestSourceIn, IngestSourceOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = ingestSourceIn,
        )
    }

    /** Get an Ingest Source by id or uid. */
    suspend fun get(sourceId: String): IngestSourceOut {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source/$sourceId")
        return client.executeRequest<Any, IngestSourceOut>("GET", url.build())
    }

    /** Update an Ingest Source. */
    suspend fun update(sourceId: String, ingestSourceIn: IngestSourceIn): IngestSourceOut {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source/$sourceId")

        return client.executeRequest<IngestSourceIn, IngestSourceOut>(
            "PUT",
            url.build(),
            reqBody = ingestSourceIn,
        )
    }

    /** Delete an Ingest Source. */
    suspend fun delete(sourceId: String) {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source/$sourceId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /**
     * Rotate the Ingest Source's Url Token.
     *
     * This will rotate the ingest source's token, which is used to construct the unique `ingestUrl`
     * for the source. Previous tokens will remain valid for 48 hours after rotation. The token can
     * be rotated a maximum of three times within the 48-hour period.
     */
    suspend fun rotateToken(
        sourceId: String,
        options: IngestSourceRotateTokenOptions = IngestSourceRotateTokenOptions(),
    ): RotateTokenOut {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source/$sourceId/token/rotate")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }
        return client.executeRequest<Any, RotateTokenOut>(
            "POST",
            url.build(),
            headers = headers.build(),
        )
    }
}
