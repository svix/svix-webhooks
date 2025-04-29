// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class ApiTokenOut(
    val createdAt: Instant,
    val expiresAt: Instant? = null,
    /** The GlobalApplicationToken's ID. */
    val id: String,
    val name: String? = null,
    val scopes: List<String>? = null,
    val token: String,
)
