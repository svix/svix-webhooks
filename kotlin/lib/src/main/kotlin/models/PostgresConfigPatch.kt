// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class PostgresConfigPatch(
    val url: String? = null,
    val password: String? = null,
    val tableName: String? = null,
    val sslRootCert: String? = null,
)
