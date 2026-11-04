// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class EventTypeUpsertIn(
    val description: String,
    /** Translated descriptions keyed by BCP 47 language tag. */
    val descriptionTranslations: Map<String, String>? = null,
    val archived: Boolean? = null,
    val deprecated: Boolean? = null,
    @Serializable(with = StringAnyMapSerializer::class)
    /** The schema for the event type for a specific version as a JSON schema. */
    val schemas: Map<String, Any>? = null,
    val featureFlags: Set<String>? = null,
    /** The event type group's name */
    val groupName: String? = null,
)
