// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class EndpointDisabledEventData(
    val appId: String,
    val appUid: String? = null,
    val endpointId: String,
    val endpointUid: String? = null,
    val failSince: Instant? = null,
    val trigger: EndpointDisabledTrigger? = null,
)
