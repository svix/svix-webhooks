// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class S3Config(
    val accessKeyId: String,
    val bucket: String,
    val region: String,
    val secretAccessKey: String,
)
