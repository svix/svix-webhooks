package com.svix.kotlin

data class SvixOptions(val debugUrl: String = DEFAULT_URL) {
    companion object {
        const val DEFAULT_URL = "https://api.svix.com"
    }
}
