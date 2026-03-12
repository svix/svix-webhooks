// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ApplicationIn(
    val metadata: Map<String, String>? = null,
    val name: String,
    val rateLimit: UShort? = null,
    /** Optional unique identifier for the application. */
    val uid: String? = null,
)
