package com.svix.kotlin

class PostOptions {
    var idempotencyKey: String? = null

    fun idempotencyKey(idempotencyKey: String) = apply { this.idempotencyKey = idempotencyKey }
}
