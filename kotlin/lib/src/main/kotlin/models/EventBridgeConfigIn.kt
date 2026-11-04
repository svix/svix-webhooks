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
     * Required (along with `secret_access_key`) if `role_arn` is blank.
     */
    val accessKeyId: String? = null,
    /**
     * Secret access key.
     *
     * Required (along with `access_key_id`) if `role_arn` is blank.
     */
    val secretAccessKey: String? = null,
    /** Role ARN for delegated authentication */
    val roleArn: String? = null,
    /**
     * Shared secret passed as the STS ExternalId.
     *
     * Can only be set if `role_arn` is Some
     */
    val externalId: String? = null,
    /**
     * The region of the EventBridge bus.
     *
     * Currently a required field, but marked as optional because we may infer it from other fields
     * in the future.
     */
    val region: String? = null,
)
