// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class MessageAttemptRecoveredEventData(
    /** The Application's ID. */
    val appId: String,
    /** The Application's UID. */
    val appUid: String? = null,
    /** The Endpoint's ID. */
    val endpointId: String,
    val lastAttempt: MessageAttemptFailedData,
    /** The Message's UID. */
    val msgEventId: String? = null,
    /** The Message's ID. */
    val msgId: String,
)
