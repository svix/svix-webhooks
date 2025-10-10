// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class GoogleCloudStorageConfig(
    val bucket: String,
    /** Google Cloud Credentials JSON Object as a string. */
    val credentials: String,
)
