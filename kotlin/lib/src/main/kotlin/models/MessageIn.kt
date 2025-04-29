// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class MessageIn(
    /**
     * Optionally creates a new application alongside the message.
     *
     * If the application id or uid that is used in the path already exists, this argument is
     * ignored.
     */
    val application: ApplicationIn? = null,
    /** List of free-form identifiers that endpoints can filter by */
    val channels: Set<String>? = null,
    /** Optional unique identifier for the message */
    val eventId: String? = null,
    /** The event type's name */
    val eventType: String,
    var payload: String,
    /**
     * Optional number of hours to retain the message payload. Note that this is mutually exclusive
     * with `payloadRetentionPeriod`.
     */
    val payloadRetentionHours: Long? = null,
    /**
     * Optional number of days to retain the message payload. Defaults to 90. Note that this is
     * mutually exclusive with `payloadRetentionHours`.
     */
    val payloadRetentionPeriod: Long? = null,
    /** List of free-form tags that can be filtered by when listing messages */
    val tags: Set<String>? = null,
    @Serializable(with = StringAnyMapSerializer::class)
    /** Extra parameters to pass to Transformations (for future use) */
    var transformationsParams: Map<String, Any>? = null,
)
