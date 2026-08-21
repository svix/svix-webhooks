// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class BulkExpungeContentsIn(
    /** Message ID or UID to delete */
    val ids: Set<String>? = null
)
