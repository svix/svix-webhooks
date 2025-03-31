package com.svix.kotlin

import SvixOptions
import okhttp3.HttpUrl.Companion.toHttpUrlOrNull

class Svix(token: String, options: SvixOptions = SvixOptions()) {
    val application: Application
    val authentication: Authentication
    val endpoint: Endpoint
    val eventType: EventType
    val ingest: Ingest
    val integration: Integration
    val management: Management
    val message: Message
    val messageAttempt: MessageAttempt
    val statistics: Statistics
    val operationalWebhook: OperationalWebhook
    val operationalWebhookEndpoint: OperationalWebhookEndpoint

    init {
        val tokenParts = token.split(".")
        if (options.baseUrl == null) {
            val region = tokenParts.last()
            when (region) {
                "us" -> options.baseUrl = "https://api.us.svix.com"
                "eu" -> options.baseUrl = "https://api.eu.svix.com"
                "in" -> options.baseUrl = "https://api.in.svix.com"
                else -> options.baseUrl = "https://api.svix.com"
            }
        }
        val parsedUrl = options.baseUrl?.toHttpUrlOrNull() ?: throw Exception("Invalid base url")
        val defaultHeaders =
            mapOf("User-Agent" to "svix-libs/${Version}/kotlin", "Authorization" to "Bearer $token")
        val httpClient = SvixHttpClient(parsedUrl, defaultHeaders, options.retrySchedule)
        application = Application(httpClient)
        authentication = Authentication(httpClient)
        endpoint = Endpoint(httpClient)
        eventType = EventType(httpClient)
        ingest = Ingest(httpClient)
        integration = Integration(httpClient)
        management = Management(httpClient)
        message = Message(httpClient)
        messageAttempt = MessageAttempt(httpClient)
        statistics = Statistics(httpClient)
        operationalWebhook = OperationalWebhook(httpClient)
        operationalWebhookEndpoint = OperationalWebhookEndpoint(httpClient)
    }
}
