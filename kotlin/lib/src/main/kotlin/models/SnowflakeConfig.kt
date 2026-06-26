// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SnowflakeConfig(
    /**
     * Snowflake account identifier, which includes both the organization and account IDs separated
     * by a hyphen.
     */
    val accountIdentifier: String,
    /**
     * Database name.
     *
     * Only required if not using transformations.
     */
    val dbName: String? = null,
    /**
     * PEM-encoded private key used for signing token-based requests to the Snowflake API.
     *
     * Beginning/end delimiters are not required.
     */
    val privateKey: String,
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
    /** The Snowflake user id. */
    val userId: String,
)
