// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class BackgroundTaskFinishedEvent2(
    @Serializable(with = StringAnyMapSerializer::class) val data: Map<String, Any>,
    val status: BackgroundTaskStatus,
    val task: BackgroundTaskType,
    val taskId: String,
)
