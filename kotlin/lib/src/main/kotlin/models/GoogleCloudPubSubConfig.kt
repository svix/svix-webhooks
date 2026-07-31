// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class GoogleCloudPubSubConfig(
    val projectId: String,
    val topicId: String,
    /** Google Cloud Credentials JSON Object as a string. */
    val credentials: String,
)
