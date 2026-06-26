// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class ClickhouseConfig(
    /** The Clickhouse database to connect to */
    val database: String? = null,
    /** Password to access Clickhouse */
    val password: String,
    /** The Clickhouse table to write to */
    val tableName: String,
    /** The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`). */
    val url: String,
    /** Username to access Clickhouse */
    val username: String,
)
