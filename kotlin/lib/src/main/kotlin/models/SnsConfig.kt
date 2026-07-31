// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SnsConfig(
    val topicArn: String,
    val region: String,
    val accessKeyId: String,
    val secretAccessKey: String,
    val endpointUrl: String? = null,
)
