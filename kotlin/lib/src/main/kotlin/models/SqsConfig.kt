// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SqsConfig(
    val accessKeyId: String,
    val endpointUrl: String? = null,
    val queueUrl: String,
    val region: String,
    val secretAccessKey: String,
)
