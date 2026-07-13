// This file is @generated
package com.svix.kotlin.models

import kotlin.time.Instant
import kotlinx.serialization.Serializable

@Serializable
data class MessageAttemptOut(
    val url: String,
    val response: String,
    val responseStatusCode: Short,
    /** Response duration in milliseconds. */
    val responseDurationMs: Long,
    val status: MessageStatus,
    val statusText: MessageStatusText,
    val triggerType: MessageAttemptTriggerType,
    /** The Message's ID. */
    val msgId: String,
    /** The Endpoint's ID. */
    val endpointId: String,
    /** The MessageAttempt's ID. */
    val id: String,
    val timestamp: Instant,
    val msg: MessageOut? = null,
)
