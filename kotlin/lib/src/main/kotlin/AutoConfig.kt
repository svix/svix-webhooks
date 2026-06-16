package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.exceptions.WebhookVerificationException
import com.svix.kotlin.internal.EndpointAutoConfig
import com.svix.kotlin.models.EndpointIn
import com.svix.kotlin.models.EndpointOut
import com.svix.kotlin.models.SubscribeIn
import java.net.http.HttpHeaders
import java.util.Base64
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import okhttp3.HttpUrl.Companion.toHttpUrlOrNull

class InvalidAutoConfigTokenException : Exception {
    constructor() : super("invalid token")
    constructor(cause: Throwable) : super("invalid token", cause)
}

class AutoConfig
@Throws(InvalidAutoConfigTokenException::class)
constructor(token: String, endpoint: EndpointIn) {
    private val appId: String
    private val endpointId: String
    private val endpoint: EndpointIn
    private val webhook: Webhook
    private val httpClient: SvixHttpClient

    init {
        val content = decodeAutoConfigTokenV1(token)

        this.webhook =
            try {
                Webhook(content.endpointSecret)
            } catch (e: IllegalArgumentException) {
                throw InvalidAutoConfigTokenException(e)
            }

        val parsedUrl =
            content.serverUrl.toHttpUrlOrNull() ?: throw InvalidAutoConfigTokenException()
        val defaultHeaders =
            mapOf(
                "User-Agent" to "svix-libs/${Version}/kotlin",
                "Authorization" to "Bearer ${content.tokenPlaintext}",
            )
        this.httpClient = SvixHttpClient(parsedUrl, defaultHeaders, listOf(50, 100, 200))

        this.appId = content.appId
        this.endpointId = content.endpointId
        this.endpoint = endpoint
    }

    /** Registers or updates the endpoint via the auto-config API. */
    @Throws(ApiException::class)
    suspend fun subscribe(): EndpointOut {
        return EndpointAutoConfig(httpClient).update(appId, endpointId, SubscribeIn(endpoint))
    }

    /** Validates the webhook payload using the endpoint signing secret from the token. */
    @Throws(WebhookVerificationException::class)
    fun verify(payload: String?, headers: HttpHeaders) {
        webhook.verify(payload, headers)
    }

    @Serializable
    internal data class AutoConfigTokenContentV1(
        @SerialName("aid") val appId: String,
        @SerialName("eid") val endpointId: String,
        @SerialName("surl") val serverUrl: String,
        @SerialName("esec") val endpointSecret: String,
        @SerialName("tok") val tokenPlaintext: String,
    )

    companion object {
        private const val AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_"

        private val json = Json { ignoreUnknownKeys = true }

        @Throws(InvalidAutoConfigTokenException::class)
        internal fun decodeAutoConfigTokenV1(token: String): AutoConfigTokenContentV1 {
            if (!token.startsWith(AUTOCONFIG_TOKEN_PREFIX_V1)) {
                throw InvalidAutoConfigTokenException()
            }
            val b64 = token.substring(AUTOCONFIG_TOKEN_PREFIX_V1.length)

            val decoded =
                try {
                    Base64.getDecoder().decode(b64)
                } catch (e: IllegalArgumentException) {
                    throw InvalidAutoConfigTokenException(e)
                }

            return try {
                json.decodeFromString(
                    AutoConfigTokenContentV1.serializer(),
                    String(decoded, Charsets.UTF_8),
                )
            } catch (e: Exception) {
                throw InvalidAutoConfigTokenException(e)
            }
        }
    }
}
