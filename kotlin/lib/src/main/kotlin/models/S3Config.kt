// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class S3Config(
    val bucket: String,
    val accessKeyId: String,
    val secretAccessKey: String,
    val region: String,
    val endpointUrl: String? = null,
)
