// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SqsConfigOut(
    val queueUrl: String,
    val region: String,
    val accessKeyId: String? = null,
    val roleArn: String? = null,
    val externalId: String? = null,
    val endpointUrl: String? = null,
)
