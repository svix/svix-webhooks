package com.svix.kotlin

data class SvixOptions(internal var wantedServerUrl: String? = null) {
    private val version = "0.64.1"

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
