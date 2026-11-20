// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EndpointOauthConfigOut(
    val grantType: Oauth2GrantTypeInOut,
    val authMethod: Oauth2AuthMethodInOut,
    val clientId: String,
    val tokenUrl: String,
    val scopes: List<String>? = null,
    val extraParams: Map<String, String>? = null,
    val extraHeaders: Map<String, String>? = null,
)
