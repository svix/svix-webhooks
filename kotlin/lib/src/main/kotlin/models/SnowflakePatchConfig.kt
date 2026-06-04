// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SnowflakePatchConfig(
    val accountIdentifier: String? = null,
    /**
     * Database name.
     *
     * Only required if not using transformations.
     */
    val dbName: String? = null,
    val privateKey: String? = null,
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
    val userId: String? = null,
)
