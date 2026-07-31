// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class BigQueryConfig(
    val projectId: String,
    val datasetId: String,
    val tableId: String,
    /** Google Cloud Credentials JSON Object as a string. */
    val credentials: String,
)
