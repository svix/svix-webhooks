// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class AzureBlobStorageConfigIn(
    val container: String,
    val account: String,
    val accessKey: String,
)
