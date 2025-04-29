// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class MessageAttemptOut(
    /** The Endpoint's ID. */
    val endpointId: String,
    /** The MessageAttempt's ID. */
    val id: String,
    val msg: MessageOut? = null,
    /** The Message's ID. */
    val msgId: String,
    val response: String,
    /** Response duration in milliseconds. */
    val responseDurationMs: Long,
    val responseStatusCode: Short,
    val status: MessageStatus,
    val timestamp: Instant,
    val triggerType: MessageAttemptTriggerType,
    val url: String,
)
