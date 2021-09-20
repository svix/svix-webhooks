package com.svix.kotlin.exceptions

import com.svix.kotlin.internal.infrastructure.ClientException
import com.svix.kotlin.internal.infrastructure.ServerException

class ApiException : Exception {
    companion object {
        internal fun wrapInternalApiException(e: Exception): Exception {
            return when (e) {
                is ServerException, is ClientException, is UnsupportedOperationException -> {
                    ApiException(e)
                }
                else -> e
            }
        }
    }

    constructor() : super()
    constructor(message: String) : super(message)
    constructor(message: String, cause: Throwable) : super(message, cause)
    constructor(cause: Throwable) : super(cause)
}
