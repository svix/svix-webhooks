package com.svix.kotlin

data class SvixOptions(
    var baseUrl: String? = null,
    val retrySchedule: List<Long> = listOf(50, 100, 200),
) {
    init {
        if (retrySchedule.count() > 5) {
            throw IllegalArgumentException("number of retries must not exceed 5")
        }
    }
}
