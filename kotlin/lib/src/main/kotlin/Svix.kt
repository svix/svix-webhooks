package com.svix.kotlin

class Svix(token: String, options: SvixOptions = SvixOptions()) {

    val tokenParts = token.split(".")
    if (options.serverUrl == null) {
        if (tokenParts.size == 2) {
          val region = tokenParts[1]
          if (region == "us") {
            options.serverUrl = "https://api.us.svix.com"
          }
          if (region == "eu") {
            options.serverUrl = "https://api.eu.svix.com"
          }
          if (region == "in") {
            options.serverUrl = "https://api.in.svix.com"
          }
        } else {
            options.serverUrl = "https://api.svix.com"
        }
    }

    val application = Application(token, options)
    val authentication = Authentication(token, options)
    val endpoint = Endpoint(token, options)
    val eventType = EventType(token, options)
    val integration = Integration(token, options)
    val message = Message(token, options)
    val messageAttempt = MessageAttempt(token, options)
}
