// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointCreatedEventData(
    val appId: String,
    val appUid: String? = null,
    val endpointId: String,
    val endpointUid: String? = null,
)
