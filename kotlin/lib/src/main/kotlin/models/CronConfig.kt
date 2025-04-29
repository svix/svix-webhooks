// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class CronConfig(
    /**
     * Override the default content-type.
     *
     * Recommended if the payload is not JSON.
     */
    val contentType: String? = null,
    val payload: String,
    val schedule: String,
)
