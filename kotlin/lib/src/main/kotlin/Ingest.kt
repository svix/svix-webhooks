// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.DashboardAccessOut
import com.svix.kotlin.models.IngestSourceConsumerPortalAccessIn
import okhttp3.Headers

data class IngestDashboardOptions(val idempotencyKey: String? = null)

class Ingest(private val client: SvixHttpClient) {
    val endpoint: IngestEndpoint = IngestEndpoint(client)

    val source: IngestSource = IngestSource(client)

    /** Get access to the Ingest Source Consumer Portal. */
    suspend fun dashboard(
        sourceId: String,
        ingestSourceConsumerPortalAccessIn: IngestSourceConsumerPortalAccessIn,
        options: IngestDashboardOptions = IngestDashboardOptions(),
    ): DashboardAccessOut {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source/$sourceId/dashboard")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<IngestSourceConsumerPortalAccessIn, DashboardAccessOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = ingestSourceConsumerPortalAccessIn,
        )
    }
}
