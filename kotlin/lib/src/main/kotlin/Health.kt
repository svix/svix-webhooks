// this file is @generated
package com.svix.kotlin

class Health(private val client: SvixHttpClient) {

    /** Verify the API server is up and running. */
    suspend fun get() {
        val url = client.newUrlBuilder().encodedPath("/api/v1/health")
        client.executeRequest<Any, Boolean>("GET", url.build())
    }
}
