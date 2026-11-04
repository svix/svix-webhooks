// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class BatchMessageIn(
    /**
     * Optional unique identifier for the message
     *
     * This should not contain the '.' character
     */
    val eventId: String? = null,
    /** The event type's name */
    val eventType: String,
    @Serializable(with = StringAnyMapSerializer::class)
    /**
     * JSON payload to send as the request body of the webhook.
     *
     * We also support sending non-JSON payloads. Please contact us for more information.
     *
     * The serialized byte-length of this value should be under 1MiB. Please contact us if you need
     * to send payloads large than 1MiB.
     */
    val payload: Map<String, Any>,
    /** List of free-form identifiers that endpoints can filter by */
    val channels: Set<String>? = null,
    /** List of free-form tags that can be filtered by when listing messages */
    val tags: Set<String>? = null,
    @Serializable(with = StringAnyMapSerializer::class)
    /** Extra parameters to pass to Transformations (for future use) */
    val transformationsParams: Map<String, Any>? = null,
    /**
     * The date and time at which the message will be delivered.
     *
     * Note that this time is best-effort-only. Must be at least one minute and no more than 24
     * hours in the future.
     */
    val deliverAt: Instant? = null,
    /**
     * Optional number of days to retain the message payload. Defaults to 90. Note that this is
     * mutually exclusive with `payloadRetentionHours`.
     */
    val payloadRetentionPeriod: Long? = null,
    /**
     * Optional number of hours to retain the message payload. Note that this is mutually exclusive
     * with `payloadRetentionPeriod`.
     */
    val payloadRetentionHours: Long? = null,
)
