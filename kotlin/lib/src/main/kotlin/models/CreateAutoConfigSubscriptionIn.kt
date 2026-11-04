// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class CreateAutoConfigSubscriptionIn(
    /**
     * The set of feature flags the created token will have access to.
     *
     * When omitted or empty, the token inherits the calling token's feature flags. When set, these
     * flags are used instead. An application token may only grant a subset of its own flags.
     */
    val featureFlags: Set<String>? = null
)
