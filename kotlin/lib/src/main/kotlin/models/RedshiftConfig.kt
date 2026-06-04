// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class RedshiftConfig(
    val accessKeyId: String,
    /** Required for provisioned clusters. */
    val clusterIdentifier: String? = null,
    /**
     * Database name.
     *
     * Only required if not using transformations.
     */
    val dbName: String? = null,
    /** Required for provisioned clusters. */
    val dbUser: String? = null,
    val region: String,
    /**
     * Schema name.
     *
     * Only used if not using transformations.
     */
    val schemaName: String? = null,
    val secretAccessKey: String,
    /**
     * Table name.
     *
     * Only required if not using transformations.
     */
    val tableName: String? = null,
    /** Required for Redshift Serverless. */
    val workgroupName: String? = null,
)
