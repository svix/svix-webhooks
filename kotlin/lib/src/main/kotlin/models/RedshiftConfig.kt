// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class RedshiftConfig(
    val accessKeyId: String,
    val secretAccessKey: String,
    val region: String,
    /** Required for provisioned clusters. */
    val clusterIdentifier: String? = null,
    /** Required for provisioned clusters. */
    val dbUser: String? = null,
    /** Required for Redshift Serverless. */
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
