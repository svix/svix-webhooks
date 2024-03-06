package com.svix.kotlin

data class SvixOptions(
    internal var wantedServerUrl: String? = null,
    val initialRetryDelayMillis: Long? = null,
    val numRetries: Int? = null,
) {
    private val version = "1.21.0"

    var serverUrl: String
        get() = this.wantedServerUrl ?: DEFAULT_URL
        set(value) {
            this.wantedServerUrl = value
        }

    companion object {
        const val DEFAULT_URL = "https://api.svix.com"
    }

    internal fun getUA(): String {
        return "svix-libs/$version/kotlin"
    }
}
