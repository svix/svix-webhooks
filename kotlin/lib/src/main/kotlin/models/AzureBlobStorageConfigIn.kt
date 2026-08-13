// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class AzureBlobStorageConfigIn(
    val container: String,
    val account: String,
    /**
     * Access key.
     *
     * Currently a required field, but marked as optional because we may add different
     * authentication in the future.
     */
    val accessKey: String? = null,
)
