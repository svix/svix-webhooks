// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class IntegrationOut(
    val name: String,
    /** The Integration's ID. */
    val id: String,
    val createdAt: Instant,
    val updatedAt: Instant,
    /** The set of feature flags the integration has access to. */
    val featureFlags: Set<String>? = null,
)
