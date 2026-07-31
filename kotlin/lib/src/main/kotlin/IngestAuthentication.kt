// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.AppPortalAccessOut
import com.svix.kotlin.models.IngestSourceConsumerPortalAccessIn
import okhttp3.Headers

data class IngestAuthenticationConsumerPortalAccessOptions(val idempotencyKey: String? = null)

class IngestAuthentication(private val client: SvixHttpClient) {
    /** Get access to the Ingest Source Consumer Portal. */
    suspend fun consumerPortalAccess(
        sourceId: String,
        ingestSourceConsumerPortalAccessIn: IngestSourceConsumerPortalAccessIn,
        options: IngestAuthenticationConsumerPortalAccessOptions =
            IngestAuthenticationConsumerPortalAccessOptions(),
    ): AppPortalAccessOut {
        val url = client.newUrlBuilder().encodedPath("/ingest/api/v1/source/$sourceId/dashboard")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<IngestSourceConsumerPortalAccessIn, AppPortalAccessOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = ingestSourceConsumerPortalAccessIn,
        )
    }
}
