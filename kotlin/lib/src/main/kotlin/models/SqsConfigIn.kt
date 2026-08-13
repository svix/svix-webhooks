// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SqsConfigIn(
    val queueUrl: String,
    /**
     * The region of the SQS queue.
     *
     * Currently a required field, but marked as optional because we may infer it from other fields
     * in the future.
     */
    val region: String? = null,
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
    val endpointUrl: String? = null,
)
