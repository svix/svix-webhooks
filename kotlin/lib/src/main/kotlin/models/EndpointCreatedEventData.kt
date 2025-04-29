// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointCreatedEventData(
    /** The Application's ID. */
    val appId: String,
    /** The Application's UID. */
    val appUid: String? = null,
    /** The Endpoint's ID. */
    val endpointId: String,
    /** The Endpoint's UID. */
    val endpointUid: String? = null,
)
