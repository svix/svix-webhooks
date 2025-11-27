// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ConnectorIn(
    val allowedEventTypes: Set<String>? = null,
    val description: String? = null,
    val featureFlags: Set<String>? = null,
    val instructions: String? = null,
    val kind: ConnectorKind? = null,
    val logo: String? = null,
    val name: String,
    val productType: ConnectorProduct? = null,
    val transformation: String,
    /** The Connector's UID. */
    val uid: String? = null,
)
