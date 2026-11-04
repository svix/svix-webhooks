// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SnsConfigIn(
    val topicArn: String,
    /**
     * The region of the SNS instance.
     *
     * Currently a required field, but marked as optional because we may infer it from other fields
     * in the future.
     */
    val region: String? = null,
    /**
     * Access key ID.
     *
     * Required (along with `secret_access_key`) if `role_arn` is None
     */
    val accessKeyId: String? = null,
    /**
     * Secret access key.
     *
     * Required (along with `access_key_id`) if `role_arn` is None
     */
    val secretAccessKey: String? = null,
    val endpointUrl: String? = null,
    /** Role ARN for delegated authentication */
    val roleArn: String? = null,
    /**
     * Shared secret passed as the STS ExternalId.
     *
     * Can only be set if `role_arn` is Some
     */
    val externalId: String? = null,
)
