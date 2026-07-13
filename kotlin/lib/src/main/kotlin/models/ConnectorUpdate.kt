// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ConnectorUpdate(
    val name: String? = null,
    val logo: String? = null,
    val description: String? = null,
    val kind: ConnectorKind? = null,
    val instructions: String? = null,
    val allowedEventTypes: Set<String>? = null,
    val transformation: String,
    val featureFlags: Set<String>? = null,
)
