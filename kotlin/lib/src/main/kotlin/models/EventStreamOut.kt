// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EventStreamOut(val data: List<EventOut>, val done: Boolean, val iterator: String)
