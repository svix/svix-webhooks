package com.svix.kotlin

data class SvixOptions(val debugUrl: String = DEFAULT_URL) {
	companion object {
		val DEFAULT_URL = "https://api.svix.com"
	}
}