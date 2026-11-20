// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ClientSecretJwtParamsIn(
    /** The base64-encoded secret used for signing the JWT. */
    val secretBase64: String,
    /** The algorithm used to sign the JWT. */
    val signingAlgorithm: OauthJwsSigningAlgorithm,
    /**
     * Optional secret identifier.
     *
     * If supplied, this will be populated in the JWT header in the `kid` field.
     */
    val secretId: String? = null,
    /**
     * Optional number of seconds after which the JWT should expire.
     *
     * Defaults to 300 seconds.
     */
    val tokenExpirySecs: ULong? = null,
)
