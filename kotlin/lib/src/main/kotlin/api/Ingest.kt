// this file is @generated
package com.svix.kotlin.api

import com.svix.kotlin.SvixHttpClient

class Ingest(private val client: SvixHttpClient) {
    val authentication: IngestAuthentication = IngestAuthentication(client)

    val endpoint: IngestEndpoint = IngestEndpoint(client)

    val source: IngestSource = IngestSource(client)
}
