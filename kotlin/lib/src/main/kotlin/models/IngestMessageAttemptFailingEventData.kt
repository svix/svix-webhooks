// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class IngestMessageAttemptFailingEventData(
    /** The Endpoint's ID. */
    val endpointId: String,
    val lastAttempt: MessageAttemptFailedData,
    /** The Message's UID. */
    val msgEventId: String? = null,
    /** The Message's ID. */
    val msgId: String,
    /** The Source's ID. */
    val sourceId: String,
)
