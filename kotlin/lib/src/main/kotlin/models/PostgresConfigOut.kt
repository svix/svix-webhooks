// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class PostgresConfigOut(
    val url: String,
    val tableName: String,
    val sslRootCert: String? = null,
)
