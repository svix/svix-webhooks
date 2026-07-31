// this file is @generated
package com.svix.kotlin

class Ingest(private val client: SvixHttpClient) {
    val authentication: IngestAuthentication = IngestAuthentication(client)

    val endpoint: IngestEndpoint = IngestEndpoint(client)

    val source: IngestSource = IngestSource(client)
}
