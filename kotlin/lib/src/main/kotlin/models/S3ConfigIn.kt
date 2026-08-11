// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class S3ConfigIn(
    val bucket: String,
    val accessKeyId: String,
    val secretAccessKey: String,
    val region: String,
    val endpointUrl: String? = null,
)
