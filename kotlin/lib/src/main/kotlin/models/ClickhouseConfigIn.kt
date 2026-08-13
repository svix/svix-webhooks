// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ClickhouseConfigIn(
    /** The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`). */
    val url: String,
    /**
     * Username to access Clickhouse.
     *
     * Currently a required field, but marked as optional because we may add different
     * authentication in the future.
     */
    val username: String? = null,
    /**
     * Password to access Clickhouse.
     *
     * Currently a required field, but marked as optional because we may add different
     * authentication in the future.
     */
    val password: String? = null,
    /** The Clickhouse database to connect to. */
    val database: String? = null,
    /** The Clickhouse table to write to. */
    val tableName: String,
)
