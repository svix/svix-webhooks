// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ListResponseStreamOut(
    val data: List<StreamOut>,
    val done: Boolean,
    val iterator: String? = null,
    val prevIterator: String? = null,
)
