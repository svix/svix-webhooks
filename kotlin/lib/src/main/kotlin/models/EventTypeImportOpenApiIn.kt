// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.StringAnyMapSerializer
import kotlinx.serialization.Serializable

@Serializable
data class EventTypeImportOpenApiIn(
    /** If `true`, return the event types that would be modified without actually modifying them. */
    val dryRun: Boolean? = null,
    /** If `true`, all existing event types that are not in the spec will be archived. */
    val replaceAll: Boolean? = null,
    @Serializable(with = StringAnyMapSerializer::class)
    /** A pre-parsed JSON spec. */
    val spec: Map<String, Any>? = null,
    /** A string, parsed by the server as YAML or JSON. */
    val specRaw: String? = null,
)
