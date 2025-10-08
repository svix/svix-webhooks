// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class StreamPortalAccessIn(
    /**
     * How long the token will be valid for, in seconds.
     *
     * Valid values are between 1 hour and 7 days. The default is 7 days.
     */
    val expiry: ULong? = null,
    /** The set of feature flags the created token will have access to. */
    val featureFlags: Set<String>? = null,
    /**
     * An optional session ID to attach to the token.
     *
     * When expiring tokens with "Expire All", you can include the session ID to only expire tokens
     * that were created with that session ID.
     */
    val sessionId: String? = null,
)
