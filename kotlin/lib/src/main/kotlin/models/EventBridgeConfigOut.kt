// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EventBridgeConfigOut(
    val eventBusName: String,
    val detailType: String,
    val accessKeyId: String,
    val region: String,
)
