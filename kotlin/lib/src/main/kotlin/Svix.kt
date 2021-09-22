package com.svix.kotlin

class Svix(token: String, options: SvixOptions = SvixOptions()) {
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
        application = Application(options)
        authentication = Authentication(options)
        endpoint = Endpoint(options)
        eventType = EventType(options)
        message = Message(options)
        messageAttempt = MessageAttempt(options)
    }
}
