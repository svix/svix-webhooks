package com.svix.kotlin

import com.svix.kotlin.SvixOptions
import okhttp3.HttpUrl.Companion.toHttpUrlOrNull

class Svix(token: String, options: SvixOptions = SvixOptions()) {
    val application: Application
    val authentication: Authentication
    val endpoint: Endpoint
    val eventType: EventType
    val ingest: Ingest
    val integration: Integration
    val message: Message
    val messageAttempt: MessageAttempt
    val statistics: Statistics
    val streaming: Streaming
    val operationalWebhook: OperationalWebhook

    init {
        val baseUrl = options.baseUrl ?: baseUrlFromToken(token)
        val parsedUrl = baseUrl.toHttpUrlOrNull() ?: throw Exception("Invalid base url")
        val httpClient = SvixHttpClient(token, parsedUrl, options.retrySchedule)

        application = Application(httpClient)
        authentication = Authentication(httpClient)
        endpoint = Endpoint(httpClient)
        eventType = EventType(httpClient)
        ingest = Ingest(httpClient)
        integration = Integration(httpClient)
        message = Message(httpClient)
        messageAttempt = MessageAttempt(httpClient)
        statistics = Statistics(httpClient)
        streaming = Streaming(httpClient)
        operationalWebhook = OperationalWebhook(httpClient)
    }
}
