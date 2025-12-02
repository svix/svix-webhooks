// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable
data class MessageAttemptLog(
    /** The Application's ID. */
    val appId: String,
    /** The Application's UID. */
    val appUid: String? = null,
    val attemptCount: UShort,
    val attemptEnd: Instant,
    /** The MessageAttempt's ID. */
    val attemptId: String,
    val attemptStart: Instant,
    /** The Endpoint's ID. */
    val endpointId: String,
    /** The event type's name */
    val eventType: String? = null,
    val httpTimes: HttpAttemptTimes? = null,
    val msgCreated: Instant,
    /** The Message's UID. */
    val msgEventId: String? = null,
    /** The Message's ID. */
    val msgId: String,
    /** The Environment's ID. */
    val orgId: String,
    val responseStatusCode: Short,
    val status: MessageStatus,
)
