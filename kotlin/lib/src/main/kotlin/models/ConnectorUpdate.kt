// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ConnectorUpdate(
    val allowedEventTypes: Set<String>? = null,
    val description: String? = null,
    val featureFlags: Set<String>? = null,
    val instructions: String? = null,
    val kind: ConnectorKind? = null,
    val logo: String? = null,
    val name: String? = null,
    val transformation: String,
)
