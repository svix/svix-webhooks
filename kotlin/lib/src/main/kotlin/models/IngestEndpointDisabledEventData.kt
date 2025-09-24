// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class IngestEndpointDisabledEventData(
    /** The Endpoint's ID. */
    val endpointId: String,
    /** The Endpoint's UID. */
    val endpointUid: String? = null,
    val failSince: Instant? = null,
    /** The Source's ID. */
    val sourceId: String,
    val trigger: EndpointDisabledTrigger? = null,
)
