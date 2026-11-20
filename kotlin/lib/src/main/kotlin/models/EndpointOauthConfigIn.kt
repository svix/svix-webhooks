// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointOauthConfigIn(
    /** The OAuth grant type. */
    val grantType: Oauth2GrantTypeInOut,
    val authMethod: Oauth2AuthMethodInOut,
    /** Optional JWT parameters. Only required for `clientSecretJwt` */
    val jwtParams: ClientSecretJwtParamsIn? = null,
    /** The client ID. Required for all authentication types. */
    val clientId: String,
    /**
     * Optional client secret. This is only used for `clientSecretBasic` and `clientSecretPost`.
     *
     * For `clientSecretBasic`, the secret will be appended to the `Authorization` header. For
     * `clientSecretPost`, this will be added to the body in a `client_secret` parameter.
     */
    val clientSecret: String? = null,
    /** The URL of the authorization server. */
    val tokenUrl: String,
    /** Optional OAuth scopes added to the request body. */
    val scopes: List<String>? = null,
    /** Extra parameters added to the request body as key-value pairs. */
    val extraParams: Map<String, String>? = null,
    /** For `refreshToken` grant type. */
    val refreshToken: String? = null,
    /** Optional headers added to the token request. */
    val extraHeaders: Map<String, String>? = null,
)
