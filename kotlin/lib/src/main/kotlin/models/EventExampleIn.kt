// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
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
    @Serializable(with = StringAnyMapSerializer::class)
    /**
     * Custom payload to send as an example
     *
     * This is only available to allow-listed customers and should otherwise not be passed. Please
     * contact us if you need access to this functionality
     */
    val payload: Map<String, Any>? = null,
)
