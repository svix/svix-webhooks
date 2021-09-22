package com.svix.kotlin

import com.svix.kotlin.internal.infrastructure.ApiClient

class Svix(token: String, options: SvixOptions = SvixOptions()) {
//    companion object {
//        val VERSION = "0.29.0"
//    }

    val application: Application
    val authentication: Authentication
    val endpoint: Endpoint
    val eventType: EventType
    val message: Message
    val messageAttempt: MessageAttempt

    init {
        // FIXME the way ApiClient is generated makes it global...
        // this means that a project can't have multiple Svix objects
        ApiClient.accessToken = token

        application = Application(options)
        authentication = Authentication(options)
        endpoint = Endpoint(options)
        eventType = EventType(options)
        message = Message(options)
        messageAttempt = MessageAttempt(options)
    }
}
