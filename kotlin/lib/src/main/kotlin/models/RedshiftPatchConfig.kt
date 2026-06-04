// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class RedshiftPatchConfig(
    val accessKeyId: String? = null,
    /**
     * Database name.
     *
     * Only required if not using transformations.
     */
    val dbName: String? = null,
    val region: String? = null,
    /**
     * Schema name.
     *
     * Only used if not using transformations.
     */
    val schemaName: String? = null,
    val secretAccessKey: String? = null,
    /**
     * Table name.
     *
     * Only required if not using transformations.
     */
    val tableName: String? = null,
)
