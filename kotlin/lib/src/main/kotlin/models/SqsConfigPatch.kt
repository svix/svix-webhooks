// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class SqsConfigPatch(
    val queueUrl: String? = null,
    val region: String? = null,
    val accessKeyId: String? = null,
    val secretAccessKey: String? = null,
    val endpointUrl: MaybeUnset<String> = MaybeUnset.Unset,
)
