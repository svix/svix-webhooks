// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.ToQueryParam
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement
import kotlinx.serialization.json.jsonPrimitive

@Serializable
enum class AppPortalCapability : ToQueryParam {
    @SerialName("ViewBase") VIEW_BASE,
    @SerialName("ViewEndpointSecret") VIEW_ENDPOINT_SECRET,
    @SerialName("ManageEndpointSecret") MANAGE_ENDPOINT_SECRET,
    @SerialName("ManageTransformations") MANAGE_TRANSFORMATIONS,
    @SerialName("CreateAttempts") CREATE_ATTEMPTS,
    @SerialName("ManageEndpoint") MANAGE_ENDPOINT;

    override fun toQueryParam() = Json.encodeToJsonElement(this).jsonPrimitive.content
}
