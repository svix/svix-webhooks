// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class AutoConfigOut(
    val createdAt: Instant,
    val token: String,
    /** The AutoConfigSubscription's ID. */
    val id: String,
)
