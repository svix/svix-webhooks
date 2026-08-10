// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class S3ConfigPatch(
    val bucket: String? = null,
    val accessKeyId: String? = null,
    val secretAccessKey: String? = null,
    val region: String? = null,
    val endpointUrl: String? = null,
)
