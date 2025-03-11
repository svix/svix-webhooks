// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class MessageAttemptRecoveredEventData(
    val appId: String,
    val appUid: String? = null,
    val endpointId: String,
    val lastAttempt: MessageAttemptFailedData,
    val msgEventId: String? = null,
    val msgId: String,
)
