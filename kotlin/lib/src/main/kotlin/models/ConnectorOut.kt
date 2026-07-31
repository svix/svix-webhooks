// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class ConnectorOut(
    /** The Connector's ID. */
    val id: String,
    /** The Environment's ID. */
    val orgId: String,
    /** The Connector's UID. */
    val uid: String? = null,
    val kind: ConnectorKind,
    val name: String,
    val logo: String? = null,
    val description: String,
    val instructions: String,
    val allowedEventTypes: Set<String>? = null,
    val transformation: String,
    val createdAt: Instant,
    val updatedAt: Instant,
    val transformationUpdatedAt: Instant,
    val featureFlags: Set<String>? = null,
    val productType: ConnectorProduct,
)
