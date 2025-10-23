// this file is @generated
package com.svix.kotlin

import com.svix.kotlin.models.ConnectorIn
import com.svix.kotlin.models.ConnectorOut
import com.svix.kotlin.models.ConnectorPatch
import com.svix.kotlin.models.ConnectorUpdate
import com.svix.kotlin.models.ListResponseConnectorOut
import com.svix.kotlin.models.Ordering
import okhttp3.Headers

data class ConnectorListOptions(
    /** Limit the number of returned items */
    val limit: ULong? = null,
    /** The iterator returned from a prior invocation */
    val iterator: String? = null,
    /** The sorting order of the returned items */
    val order: Ordering? = null,
)

data class ConnectorCreateOptions(val idempotencyKey: String? = null)

class Connector(private val client: SvixHttpClient) {
    /** List all connectors for an application. */
    suspend fun list(
        options: ConnectorListOptions = ConnectorListOptions()
    ): ListResponseConnectorOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/connector")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.iterator?.let { url.addQueryParameter("iterator", it) }
        options.order?.let { url.addQueryParameter("order", serializeQueryParam(it)) }
        return client.executeRequest<Any, ListResponseConnectorOut>("GET", url.build())
    }

    /** Create a new connector. */
    suspend fun create(
        connectorIn: ConnectorIn,
        options: ConnectorCreateOptions = ConnectorCreateOptions(),
    ): ConnectorOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/connector")
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        return client.executeRequest<ConnectorIn, ConnectorOut>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = connectorIn,
        )
    }

    /** Get a connector. */
    suspend fun get(connectorId: String): ConnectorOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/connector/$connectorId")
        return client.executeRequest<Any, ConnectorOut>("GET", url.build())
    }

    /** Update a connector. */
    suspend fun update(connectorId: String, connectorUpdate: ConnectorUpdate): ConnectorOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/connector/$connectorId")

        return client.executeRequest<ConnectorUpdate, ConnectorOut>(
            "PUT",
            url.build(),
            reqBody = connectorUpdate,
        )
    }

    /** Delete a connector. */
    suspend fun delete(connectorId: String) {
        val url = client.newUrlBuilder().encodedPath("/api/v1/connector/$connectorId")
        client.executeRequest<Any, Boolean>("DELETE", url.build())
    }

    /** Partially update a connector. */
    suspend fun patch(connectorId: String, connectorPatch: ConnectorPatch): ConnectorOut {
        val url = client.newUrlBuilder().encodedPath("/api/v1/connector/$connectorId")

        return client.executeRequest<ConnectorPatch, ConnectorOut>(
            "PATCH",
            url.build(),
            reqBody = connectorPatch,
        )
    }
}
