// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class BulkExpungeContentsOut(
    /** Results of expunging (by ID) */
    val results: Map<String, BulkExpungeStatus>
)
