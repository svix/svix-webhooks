// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class AppPortalAccessIn(
    /**
     * Optionally creates a new application while generating the access link.
     *
     * If the application id or uid that is used in the path already exists, this argument is
     * ignored.
     */
    val application: ApplicationIn? = null,
    /**
     * How long the token will be valid for, in seconds.
     *
     * Valid values are between 1 hour and 7 days. The default is 7 days.
     */
    val expiry: ULong? = null,
    /** The set of feature flags the created token will have access to. */
    val featureFlags: Set<String>? = null,
    /** Whether the app portal should be in read-only mode. */
    val readOnly: Boolean? = null,
    /**
     * An optional session ID to attach to the token.
     *
     * When expiring tokens with "Expire All", you can include the session ID to only expire tokens
     * that were created with that session ID.
     */
    val sessionId: String? = null,
)
