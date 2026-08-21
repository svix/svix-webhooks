// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SnowflakeConfigOut(
    val accountIdentifier: String,
    val userId: String,
    /**
     * Database name.
     *
     * Only required if not using transformations.
     */
    val dbName: String? = null,
    /**
     * Schema name.
     *
     * Only required if not using transformations.
     */
    val schemaName: String? = null,
    /**
     * Table name.
     *
     * Only required if not using transformations.
     */
    val tableName: String? = null,
)
