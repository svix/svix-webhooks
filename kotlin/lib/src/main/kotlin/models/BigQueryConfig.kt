// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class BigQueryConfig(
    /** Google Cloud Credentials JSON Object as a string. */
    val credentials: String,
    val datasetId: String,
    val projectId: String,
    val tableId: String,
)
