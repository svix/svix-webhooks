package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.exceptions.WebhookVerificationException
import com.svix.kotlin.internal.EndpointAutoConfigDeprecated
import com.svix.kotlin.internal.EndpointAutoconfig
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
    constructor(detail: String) : super(detail)
    constructor(cause: Throwable) : super("invalid token", cause)
}

class AutoConfig
@Throws(InvalidAutoConfigTokenException::class)
constructor(token: String, endpoint: EndpointIn) {
    private val appId: String
    private val endpointId: String?
    private val autoconfigId: String?
    private val endpoint: EndpointIn
    private val webhook: Webhook
    private val httpClient: SvixHttpClient

    init {
        val content = decodeAutoConfigToken(token)

        this.webhook =
            try {
                Webhook(content.endpointSecret)
            } catch (e: IllegalArgumentException) {
                throw InvalidAutoConfigTokenException(e)
            }

        val parsedUrl =
            content.serverUrl.toHttpUrlOrNull() ?: throw InvalidAutoConfigTokenException()
        this.httpClient = SvixHttpClient(content.tokenPlaintext, parsedUrl, listOf(50, 100, 200))

        this.appId = content.appId
        this.endpointId = content.endpointId
        this.autoconfigId = content.autoconfigId
        this.endpoint = endpoint
    }

    /** Registers or updates the endpoint via the auto-config API. */
    @Throws(ApiException::class)
    suspend fun subscribe(): EndpointOut {
        return if (autoconfigId != null) {
            EndpointAutoconfig(httpClient).subscribe(appId, autoconfigId, endpoint)
        } else {
            EndpointAutoConfigDeprecated(httpClient)
                .update(appId, endpointId as String, SubscribeIn(endpoint))
        }
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

    @Serializable
    internal data class AutoConfigTokenContentV2(
        @SerialName("aid") val appId: String,
        @SerialName("sid") val autoconfigId: String,
        @SerialName("surl") val serverUrl: String,
        @SerialName("esec") val endpointSecret: String,
        @SerialName("tok") val tokenPlaintext: String,
    )

    internal data class DecodedAutoConfigToken(
        val appId: String,
        val serverUrl: String,
        val endpointSecret: String,
        val tokenPlaintext: String,
        val endpointId: String? = null,
        val autoconfigId: String? = null,
    )

    companion object {
        private const val AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_"
        private const val AUTOCONFIG_TOKEN_PREFIX_V2 = "auto_v2_"
        private const val UNSUPPORTED_TOKEN_VERSION =
            "Unsupported token version. You might need to update the Svix SDK to use this token"

        private val json = Json { ignoreUnknownKeys = true }

        private fun parseTokenPayload(token: String, prefix: String): String {
            if (!token.startsWith(prefix)) {
                throw InvalidAutoConfigTokenException(UNSUPPORTED_TOKEN_VERSION)
            }
            val b64 = token.substring(prefix.length)

            val decoded =
                try {
                    Base64.getDecoder().decode(b64)
                } catch (e: IllegalArgumentException) {
                    throw InvalidAutoConfigTokenException(e)
                }

            return String(decoded, Charsets.UTF_8)
        }

        @Throws(InvalidAutoConfigTokenException::class)
        internal fun decodeAutoConfigTokenV1(token: String): AutoConfigTokenContentV1 {
            val payload = parseTokenPayload(token, AUTOCONFIG_TOKEN_PREFIX_V1)
            return try {
                json.decodeFromString(AutoConfigTokenContentV1.serializer(), payload)
            } catch (e: Exception) {
                throw InvalidAutoConfigTokenException(e)
            }
        }

        @Throws(InvalidAutoConfigTokenException::class)
        internal fun decodeAutoConfigTokenV2(token: String): AutoConfigTokenContentV2 {
            val payload = parseTokenPayload(token, AUTOCONFIG_TOKEN_PREFIX_V2)
            return try {
                json.decodeFromString(AutoConfigTokenContentV2.serializer(), payload)
            } catch (e: Exception) {
                throw InvalidAutoConfigTokenException(e)
            }
        }

        @Throws(InvalidAutoConfigTokenException::class)
        internal fun decodeAutoConfigToken(token: String): DecodedAutoConfigToken {
            if (token.startsWith(AUTOCONFIG_TOKEN_PREFIX_V1)) {
                val content = decodeAutoConfigTokenV1(token)
                return DecodedAutoConfigToken(
                    appId = content.appId,
                    serverUrl = content.serverUrl,
                    endpointSecret = content.endpointSecret,
                    tokenPlaintext = content.tokenPlaintext,
                    endpointId = content.endpointId,
                )
            }
            if (token.startsWith(AUTOCONFIG_TOKEN_PREFIX_V2)) {
                val content = decodeAutoConfigTokenV2(token)
                return DecodedAutoConfigToken(
                    appId = content.appId,
                    serverUrl = content.serverUrl,
                    endpointSecret = content.endpointSecret,
                    tokenPlaintext = content.tokenPlaintext,
                    autoconfigId = content.autoconfigId,
                )
            }
            throw InvalidAutoConfigTokenException(UNSUPPORTED_TOKEN_VERSION)
        }
    }
}
