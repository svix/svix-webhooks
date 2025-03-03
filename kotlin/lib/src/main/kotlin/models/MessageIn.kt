// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class MessageIn(
    val application: ApplicationIn? = null,
    val channels: Set<String>? = null,
    val eventId: String? = null,
    val eventType: String,
    var payload: String,
    val payloadRetentionHours: Long? = null,
    val payloadRetentionPeriod: Long? = null,
    val tags: Set<String>? = null,
    @Serializable(with = StringAnyMapSerializer::class)
    var transformationsParams: Map<String, Any>? = null,
)
