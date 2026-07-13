// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class CronConfig(
    val schedule: String,
    val payload: String,
    /**
     * Override the default content-type.
     *
     * Recommended if the payload is not JSON.
     */
    val contentType: String? = null,
)
