// This file is @generated
package com.svix.kotlin.models

import com.svix.kotlin.MaybeUnset
import kotlinx.serialization.Serializable

@Serializable
data class RedshiftConfigPatch(
    val accessKeyId: String? = null,
    val secretAccessKey: String? = null,
    val region: String? = null,
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
    val schemaName: MaybeUnset<String> = MaybeUnset.Unset,
    /**
     * Table name.
     *
     * Only required if not using transformations.
     */
    val tableName: String? = null,
)
