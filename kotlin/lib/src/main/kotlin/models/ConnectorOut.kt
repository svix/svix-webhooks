// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class ConnectorOut(
    val createdAt: Instant,
    val description: String,
    val featureFlag: String? = null,
    val filterTypes: Set<String>? = null,
    /** The Connector's ID. */
    val id: String,
    val instructions: String,
    val instructionsLink: String? = null,
    val kind: ConnectorKind,
    val logo: String,
    val name: String,
    /** The Environment's ID. */
    val orgId: String,
    val transformation: String,
    val updatedAt: Instant,
)
