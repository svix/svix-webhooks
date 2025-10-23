// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.EnvironmentIn
import com.svix.kotlin.models.EnvironmentOut
import okhttp3.Headers

data class EnvironmentExportOptions(val idempotencyKey: String? = null)

data class EnvironmentImportOptions(val idempotencyKey: String? = null)

class Environment(private val client: SvixHttpClient) {
    /**
     * Download a JSON file containing all org-settings and event types.
     *
     * Note that the schema for [`EnvironmentOut`] is subject to change. The fields herein are
     * provided for convenience but should be treated as JSON blobs.
     */
    suspend fun export(
        options: EnvironmentExportOptions = EnvironmentExportOptions()
    ): EnvironmentOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/environment/export")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }
        return client.executeRequest<Any, EnvironmentOut>(
            "POST",
            url.build(),
            headers = headers.build(),
        )
    }

    /**
     * Import a configuration into the active organization.
     *
     * It doesn't delete anything, only adds / updates what was passed to it.
     *
     * Note that the schema for [`EnvironmentIn`] is subject to change. The fields herein are
     * provided for convenience but should be treated as JSON blobs.
     */
    suspend fun import(
        environmentIn: EnvironmentIn,
        options: EnvironmentImportOptions = EnvironmentImportOptions(),
    ) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/environment/import")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        client.executeRequest<EnvironmentIn, Boolean>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = environmentIn,
        )
    }
}
