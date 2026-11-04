// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class RabbitMqConfigIn(
    /**
     * URI to connect to
     *
     * Note that the VHost must be percent-escaped, so a default URI would look like
     * `amqp://user:pass@host/%2F`
     */
    val uri: String,
    /** Routing key for message dispatch */
    val routingKey: String,
    /**
     * If true, then dispatches will fail if there is no attached queue; if false, they are silently
     * dropped (this was previously the default)
     */
    val mandatory: Boolean? = null,
)
