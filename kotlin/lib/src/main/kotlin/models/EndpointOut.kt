// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class EndpointOut(
    /** List of message channels this endpoint listens to (omit for all). */
    val channels: Set<String>? = null,
    val createdAt: Instant,
    /** An example endpoint name. */
    val description: String,
    val disabled: Boolean? = null,
    val filterTypes: Set<String>? = null,
    /** The Endpoint's ID. */
    val id: String,
    val metadata: Map<String, String>,
    val rateLimit: UShort? = null,
    /** Optional unique identifier for the endpoint. */
    val uid: String? = null,
    val updatedAt: Instant,
    val url: String,
    val version: Int,
)
