package com.svix.kotlin

data class SvixOptions(
    var baseUrl: String? = null,
    val retrySchedule: List<Long> = listOf(50, 100, 200),
    var serverUrl: String? = null,
) {
    init {
        val rawUrl = serverUrl ?: baseUrl
        baseUrl = rawUrl?.trimEnd('/')
        serverUrl = baseUrl
        if (retrySchedule.count() > 5) {
            throw IllegalArgumentException("number of retries must not exceed 5")
        }
    }
}
