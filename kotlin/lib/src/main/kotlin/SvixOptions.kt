package com.svix.kotlin

data class SvixOptions(val debugUrl: String = DEFAULT_URL) {
    val VERSION = "0.29.0"

    companion object {
        const val DEFAULT_URL = "https://api.svix.com"
    }
}
