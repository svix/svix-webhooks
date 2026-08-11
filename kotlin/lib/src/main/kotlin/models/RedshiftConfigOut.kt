// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class RedshiftConfigOut(
    val accessKeyId: String,
    val region: String,
    val clusterIdentifier: String? = null,
    val dbUser: String? = null,
    val workgroupName: String? = null,
    /**
     * Database name.
     *
     * Only required if not using transformations.
     */
    val dbName: String? = null,
    /**
     * Schema name.
     *
     * Only used if not using transformations.
     */
    val schemaName: String? = null,
    /**
     * Table name.
     *
     * Only required if not using transformations.
     */
    val tableName: String? = null,
)
