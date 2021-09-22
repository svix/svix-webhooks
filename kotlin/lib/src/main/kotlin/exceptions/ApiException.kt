package com.svix.kotlin.exceptions

import com.svix.kotlin.internal.infrastructure.ClientException
import com.svix.kotlin.internal.infrastructure.ServerException

class ApiException internal constructor(message: String? = null, val statusCode: Int = -1) : RuntimeException(message) {
    companion object {
        internal fun wrap(e: Exception): Exception {
            return when (e) {
                is ServerException -> {
                    ApiException(e.message, e.statusCode)
                }
                is ClientException -> {
                    ApiException(e.message, e.statusCode)
                }
                else -> e
            }
        }
    }
}
