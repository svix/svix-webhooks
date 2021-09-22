package com.svix.kotlin

import com.svix.kotlin.internal.infrastructure.ApiClient

class Svix(token: String, svixOptions: SvixOptions = SvixOptions()) {
    companion object {
        val VERSION = "0.29.0"
    }

    private val application: Application
    private val authentication: Authentication
    private val endpoint: Endpoint
    private val eventType: EventType
    private val message: Message
    private val messageAttempt: MessageAttempt

    init {
        ApiClient.accessToken = token

        application = Application(svixOptions.debugUrl)
        authentication = Authentication(svixOptions.debugUrl)
        endpoint = Endpoint(svixOptions.debugUrl)
        eventType = EventType(svixOptions.debugUrl)
        message = Message(svixOptions.debugUrl)
        messageAttempt = MessageAttempt(svixOptions.debugUrl)
    }
}
