package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import kotlin.random.Random
import kotlin.random.nextULong
import kotlinx.coroutines.delay
import kotlinx.serialization.json.Json
import okhttp3.*
import okhttp3.HttpUrl.Companion.toHttpUrlOrNull
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.RequestBody.Companion.toRequestBody
import java.io.IOException
import java.time.Duration
import java.util.UUID

internal fun baseUrlFromToken(token: String): String {
    val region = token.split(".").last()
    return when (region) {
        "us" -> "https://api.us.svix.com"
        "eu" -> "https://api.eu.svix.com"
        "in" -> "https://api.in.svix.com"
        "ca" -> "https://api.ca.svix.com"
        "au" -> "https://api.au.svix.com"
        else -> "https://api.svix.com"
    }
}

open class SvixHttpClient
internal constructor(
    token: String,
    rawBaseUrl: HttpUrl,
    private val retrySchedule: List<Long>,
) {
    val baseUrl: HttpUrl =
        if (rawBaseUrl.encodedPath.length > 1 && rawBaseUrl.encodedPath.endsWith('/')) {
            val trimmedPath = rawBaseUrl.encodedPath.trimEnd('/')
            rawBaseUrl.newBuilder().encodedPath(if (trimmedPath.isEmpty()) "/" else trimmedPath).build()
        } else {
            rawBaseUrl
        }
    private val defaultHeaders: Map<String, String>
    private val client: OkHttpClient =
        OkHttpClient.Builder().callTimeout(Duration.ofSeconds(15)).build()
    private val jsonDeserializer: Json = Json { ignoreUnknownKeys = true }

    init {
        var userAgent = "svix-libs/${Version}/kotlin kotlin/${KotlinVersion.CURRENT}"
        val jreVersion = System.getProperty("java.runtime.version")
        if (jreVersion != null) {
            userAgent += " jre/${jreVersion}"
        }

        defaultHeaders = mapOf("User-Agent" to userAgent, "Authorization" to "Bearer $token")
    }

    fun newUrlBuilder(): HttpUrl.Builder {
        return HttpUrl.Builder().scheme(baseUrl.scheme).host(baseUrl.host).port(baseUrl.port)
    }

    internal suspend inline fun <reified Req, reified Res> executeRequest(
        method: String,
        url: HttpUrl,
        headers: Headers? = null,
        reqBody: Req? = null,
    ): Res {
        val reqBuilder = Request.Builder().url(url)
        if (reqBody != null) {
            reqBuilder.method(
                method,
                Json.encodeToString(reqBody).toRequestBody("application/json".toMediaType()),
            )
        } else if (method == "POST" || method == "PUT" || method == "PATCH") {
            reqBuilder.method(method, ByteArray(0).toRequestBody())
        } else {
            reqBuilder.method(method, null)
        }

        for ((k, v) in defaultHeaders) {
            reqBuilder.addHeader(k, v)
        }
        if (headers != null) {
            for ((k, v) in headers) {
                reqBuilder.addHeader(k, v)
            }
        }

        if (headers?.get("idempotency-key") == null && method == "POST") {
            val uuid = UUID.randomUUID().toString()
            reqBuilder.addHeader("idempotency-key", "auto_" + uuid)
        }

        reqBuilder.addHeader("svix-req-id", Random.nextULong().toString())

        val request = reqBuilder.build()
        val res = executeRequestWithRetry(request)

        // if body is null panic
        if (res.body == null) {
            throw ApiException("Body is null", res.code)
        }
        val bodyString = res.body!!.string()
        if (res.code == 204) {
            return jsonDeserializer.decodeFromString<Res>("true")
        }
        if (res.code in 200..299) {
            return jsonDeserializer.decodeFromString<Res>(bodyString)
        }
        throw ApiException("Non 200 status code ${res.code}", res.code, bodyString)
    }

    suspend fun executeRequestWithRetry(request: Request): Response {
        var currentRequest = request
        retrySchedule.forEachIndexed { index, sleepTime ->
            try {
                val res = client.newCall(currentRequest).execute()
                if (res.code < 500) {
                    return res
                }
                res.close()
            } catch (e: IOException) {
                // Network errors (timeouts, connection failures, etc.) are retried
            }

            delay(sleepTime)
            currentRequest =
                request.newBuilder().header("svix-retry-count", (index + 1).toString()).build()
        }
        return client.newCall(currentRequest).execute()
    }
}
