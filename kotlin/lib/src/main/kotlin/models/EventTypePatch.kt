// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import com.svix.kotlin.MaybeUnsetStringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class EventTypePatch(
    val description: String? = null,
    /** Translated descriptions keyed by BCP 47 language tag. */
    val descriptionTranslations: MaybeUnset<Map<String, String>> = MaybeUnset.Unset,
    val archived: Boolean? = null,
    val deprecated: Boolean? = null,
    @Serializable(with = MaybeUnsetStringAnyMapSerializer::class)
    val schemas: MaybeUnset<Map<String, Any>> = MaybeUnset.Unset,
    val featureFlags: MaybeUnset<Set<String>> = MaybeUnset.Unset,
    /** The event type group's name */
    val groupName: MaybeUnset<String> = MaybeUnset.Unset,
)
