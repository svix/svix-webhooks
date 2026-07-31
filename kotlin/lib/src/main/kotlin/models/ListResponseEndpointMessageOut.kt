// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ListResponseEndpointMessageOut(
    val data: List<EndpointMessageOut>,
    val iterator: String? = null,
    val prevIterator: String? = null,
    val done: Boolean,
)
