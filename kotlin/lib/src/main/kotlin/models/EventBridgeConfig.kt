// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EventBridgeConfig(
    val accessKeyId: String,
    /** Free-form string, with a maximum of 128 characters */
    val detailType: String? = null,
    /** The name or ARN of the event bus to receive the event */
    val eventBusName: String,
    val region: String,
    val secretAccessKey: String,
)
