// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ApplicationTokenExpireIn(
    /** How many seconds until the old key is expired. */
    val expiry: Long? = null,
    /**
     * An optional list of session ids.
     *
     * If any session ids are specified, only Application tokens created with that session id will
     * be expired.
     */
    val sessionIds: List<String>? = null,
)
