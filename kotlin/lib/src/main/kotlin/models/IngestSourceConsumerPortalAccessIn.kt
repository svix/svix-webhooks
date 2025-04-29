// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class IngestSourceConsumerPortalAccessIn(
    /**
     * How long the token will be valid for, in seconds.
     *
     * Valid values are between 1 hour and 7 days. The default is 7 days.
     */
    val expiry: ULong? = null,
    /** Whether the app portal should be in read-only mode. */
    val readOnly: Boolean? = null,
)
