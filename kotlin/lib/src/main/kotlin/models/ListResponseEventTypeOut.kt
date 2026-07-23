// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ListResponseEventTypeOut(
    val data: List<EventTypeOut>,
    val iterator: String? = null,
    val prevIterator: String? = null,
    val done: Boolean,
)
