package com.svix.kotlin

class Svix(token: String, options: SvixOptions = SvixOptions()) {
    val application = Application(token, options)
    val authentication = Authentication(token, options)
    val endpoint = Endpoint(token, options)
    val eventType = EventType(token, options)
    val message = Message(token, options)
    val messageAttempt = MessageAttempt(token, options)
}
