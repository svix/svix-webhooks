// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointMtlsConfigIn(
    /** A PEM encoded private key and X.509 certificate to identify the webhook sender. */
    val identity: String
)
