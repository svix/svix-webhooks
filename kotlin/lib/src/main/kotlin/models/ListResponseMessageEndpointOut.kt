// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ListResponseMessageEndpointOut(
    val data: List<MessageEndpointOut>,
    val iterator: String? = null,
    val prevIterator: String? = null,
    val done: Boolean,
)
