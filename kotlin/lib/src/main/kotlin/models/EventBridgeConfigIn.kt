// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class EventBridgeConfigIn(
    /** The name or ARN of the event bus to receive the event */
    val eventBusName: String,
    /** Free-form string, with a maximum of 128 characters */
    val detailType: String? = null,
    /**
     * Access key ID.
     *
     * Currently a required field, but marked as optional because we may add different
     * authentication in the future.
     */
    val accessKeyId: String? = null,
    /**
     * Secret access key.
     *
     * Currently a required field, but marked as optional because we may add different
     * authentication in the future.
     */
    val secretAccessKey: String? = null,
    /**
     * The region of the EventBridge bus.
     *
     * Currently a required field, but marked as optional because we may infer it from other fields
     * in the future.
     */
    val region: String? = null,
)
