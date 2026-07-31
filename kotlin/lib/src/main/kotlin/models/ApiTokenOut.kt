// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class ApiTokenOut(
    val token: String,
    val id: String,
    val name: String? = null,
    val createdAt: Instant,
    val expiresAt: Instant? = null,
    val scopes: List<String>? = null,
)
