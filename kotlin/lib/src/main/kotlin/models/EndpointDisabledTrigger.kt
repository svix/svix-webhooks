// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.ToQueryParam
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement
import kotlinx.serialization.json.jsonPrimitive

@Serializable
enum class EndpointDisabledTrigger : ToQueryParam {
    @SerialName("manual") MANUAL,
    @SerialName("automatic") AUTOMATIC;

    override fun toQueryParam() = Json.encodeToJsonElement(this).jsonPrimitive.content
}
