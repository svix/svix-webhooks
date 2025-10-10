// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class CreateStreamEventsIn(
    val events: List<EventIn>,
    /**
     * Optionally creates a new Stream alongside the events.
     *
     * If the stream id or uid that is used in the path already exists, this argument is ignored.
     */
    val stream: StreamIn? = null,
)
