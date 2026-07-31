// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SqsConfig(
    val queueUrl: String,
    val region: String,
    val accessKeyId: String,
    val secretAccessKey: String,
    val endpointUrl: String? = null,
)
