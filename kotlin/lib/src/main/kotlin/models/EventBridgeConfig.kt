// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EventBridgeConfig(
    /** The name or ARN of the event bus to receive the event */
    val eventBusName: String,
    /** Free-form string, with a maximum of 128 characters */
    val detailType: String? = null,
    val accessKeyId: String,
    val secretAccessKey: String,
    val region: String,
)
