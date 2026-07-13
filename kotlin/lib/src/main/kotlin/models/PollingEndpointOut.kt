// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class PollingEndpointOut(
    val data: List<PollingEndpointMessageOut>,
    val iterator: String,
    val done: Boolean,
)
