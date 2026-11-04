// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class S3ConfigIn(
    val bucket: String,
    /**
     * Access key ID.
     *
     * Required (along with `secret_access_key`) if `role_arn` is null
     */
    val accessKeyId: String? = null,
    /**
     * Secret access key.
     *
     * Required (along with `access_key_id`) if `role_arn` is null
     */
    val secretAccessKey: String? = null,
    /**
     * The region of the S3 bucket
     *
     * Currently a required field, but marked as optional because we may infer it from other fields
     * in the future.
     */
    val region: String? = null,
    val endpointUrl: String? = null,
    /** Role ARN for delegated authentication */
    val roleArn: String? = null,
    /**
     * Shared secret passed as the STS ExternalId.
     *
     * Recommended if `role_arn` is not null.
     */
    val externalId: String? = null,
)
