// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class ConnectorPatch(
    val allowedEventTypes: MaybeUnset<Set<String>> = MaybeUnset.Unset,
    val description: String? = null,
    val featureFlags: MaybeUnset<Set<String>> = MaybeUnset.Unset,
    val instructions: String? = null,
    val kind: ConnectorKind? = null,
    val logo: MaybeUnset<String> = MaybeUnset.Unset,
    val name: String? = null,
    val transformation: String? = null,
)
