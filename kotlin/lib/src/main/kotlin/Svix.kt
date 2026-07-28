// this file is @generated
package com.svix.kotlin

import okhttp3.HttpUrl.Companion.toHttpUrlOrNull

class Svix(token: String, options: SvixOptions = SvixOptions()) {
    val application: Application
    val authentication: Authentication
    val backgroundTask: BackgroundTask
    val connector: Connector
    val endpoint: Endpoint
    val environment: Environment
    val eventType: EventType
    val health: Health
    val ingest: Ingest
    val integration: Integration
    val message: Message
    val messageAttempt: MessageAttempt
    val operationalWebhook: OperationalWebhook
    val statistics: Statistics
    val streaming: Streaming

    init {
        val baseUrl = options.baseUrl ?: baseUrlFromToken(token)
        val parsedUrl = baseUrl.toHttpUrlOrNull() ?: throw Exception("Invalid base url")
        val httpClient = SvixHttpClient(token, parsedUrl, options.retrySchedule)

        application = Application(httpClient)
        authentication = Authentication(httpClient)
        backgroundTask = BackgroundTask(httpClient)
        connector = Connector(httpClient)
        endpoint = Endpoint(httpClient)
        environment = Environment(httpClient)
        eventType = EventType(httpClient)
        health = Health(httpClient)
        ingest = Ingest(httpClient)
        integration = Integration(httpClient)
        message = Message(httpClient)
        messageAttempt = MessageAttempt(httpClient)
        operationalWebhook = OperationalWebhook(httpClient)
        statistics = Statistics(httpClient)
        streaming = Streaming(httpClient)
    }
}
