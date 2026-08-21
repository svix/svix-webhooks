// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class RedshiftConfigIn(
    /**
     * Access key ID.
     *
     * Currently a required field, but marked as optional because we may add different
     * authentication in the future.
     */
    val accessKeyId: String? = null,
    /**
     * Secret access key.
     *
     * Currently a required field, but marked as optional because we may add different
     * authentication in the future.
     */
    val secretAccessKey: String? = null,
    /**
     * The region of the Redshift DB.
     *
     * Currently a required field, but marked as optional because we may infer it from other fields
     * in the future.
     */
    val region: String? = null,
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
