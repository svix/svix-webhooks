// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ConnectorIn(
    val name: String,
    /** The Connector's UID. */
    val uid: String? = null,
    val logo: String? = null,
    val description: String? = null,
    val kind: ConnectorKind? = null,
    val instructions: String? = null,
    val allowedEventTypes: Set<String>? = null,
    val transformation: String,
    val featureFlags: Set<String>? = null,
    val productType: ConnectorProduct? = null,
)
