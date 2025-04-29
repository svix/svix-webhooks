// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class IntegrationOut(
    val createdAt: Instant,
    /** The set of feature flags the integration has access to. */
    val featureFlags: Set<String>? = null,
    /** The Integration's ID. */
    val id: String,
    val name: String,
    val updatedAt: Instant,
)
