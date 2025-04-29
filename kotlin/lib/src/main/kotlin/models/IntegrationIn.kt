// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class IntegrationIn(
    /** The set of feature flags the integration will have access to. */
    val featureFlags: Set<String>? = null,
    val name: String,
)
