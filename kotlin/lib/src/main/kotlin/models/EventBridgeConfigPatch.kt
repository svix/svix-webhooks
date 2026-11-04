// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EventBridgeConfigPatch(
    val eventBusName: String? = null,
    val detailType: String? = null,
    val accessKeyId: String? = null,
    val secretAccessKey: String? = null,
    val roleArn: String? = null,
    val externalId: String? = null,
    val region: String? = null,
)
