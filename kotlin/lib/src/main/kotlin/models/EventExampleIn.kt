// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EventExampleIn(
    /** The event type's name */
    val eventType: String,
    /**
     * If the event type schema contains an array of examples, chooses which one to send.
     *
     * Defaults to the first example. Ignored if the schema doesn't contain an array of examples.
     */
    val exampleIndex: ULong? = null,
)
