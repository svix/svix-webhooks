package com.svix.kotlin.exceptions

import com.svix.kotlin.internal.infrastructure.ClientError
import com.svix.kotlin.internal.infrastructure.ClientException
import com.svix.kotlin.internal.infrastructure.Response
import com.svix.kotlin.internal.infrastructure.ServerError
import com.svix.kotlin.internal.infrastructure.ServerException

class ApiException internal constructor(message: String? = null, val statusCode: Int = -1, val body: String? = null) : RuntimeException(
    message,
) {
    companion object {
        private fun extractBody(response: Response?): String? {
            return when (response) {
                is ClientError<*> -> {
                    val body = response.body
                    return if (body is String) body else null
                }
                is ServerError<*> -> {
                    val body = response.body
                    return if (body is String) body else null
                }
                else -> null
            }
        }

        internal fun wrap(e: Exception): Exception {
            return when (e) {
                is ServerException -> {
                    ApiException(e.message, e.statusCode, extractBody(e.response))
                }
                is ClientException -> {
                    ApiException(e.message, e.statusCode, extractBody(e.response))
                }
                else -> e
            }
        }
    }
}
