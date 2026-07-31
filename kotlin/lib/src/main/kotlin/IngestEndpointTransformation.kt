// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.IngestEndpointTransformationOut
import com.svix.kotlin.models.IngestEndpointTransformationPatch

class IngestEndpointTransformation(private val client: SvixHttpClient) {
    /** Get the transformation code associated with this ingest endpoint. */
    suspend fun transformation(
        sourceId: String,
        endpointId: String,
    ): IngestEndpointTransformationOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/$sourceId/endpoint/$endpointId/transformation")
        return client.executeRequest<Any, IngestEndpointTransformationOut>("GET", url.build())
    }

    /** Set or unset the transformation code associated with this ingest endpoint. */
    suspend fun patch(
        sourceId: String,
        endpointId: String,
        ingestEndpointTransformationPatch: IngestEndpointTransformationPatch,
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/ingest/api/v1/source/$sourceId/endpoint/$endpointId/transformation")

        client.executeRequest<IngestEndpointTransformationPatch, Boolean>(
            "PATCH",
            url.build(),
            reqBody = ingestEndpointTransformationPatch,
        )
    }
}
