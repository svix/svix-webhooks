package com.svix.kotlin

import com.svix.kotlin.SvixOptions
import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.models.ApplicationIn
import com.svix.kotlin.models.EndpointIn
import com.svix.kotlin.models.EndpointPatch
import com.svix.kotlin.models.EventTypeIn
import com.svix.kotlin.models.MessageIn
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.JsonPrimitive
import okhttp3.HttpUrl.Companion.toHttpUrlOrNull

class BasicTest {
    companion object {
        private val AUTH_TOKEN: String? = System.getenv("SVIX_TOKEN")
        private val SERVER_URL: String? = System.getenv("SVIX_SERVER_URL")
    }

    private fun getTestClient(): Svix? =
        if (AUTH_TOKEN == null || SERVER_URL == null) {
            // TODO: figure out how to log a warning here to help teach about tests that skip when
            // the env vars are unset.
            null
        } else {
            Svix(AUTH_TOKEN, SvixOptions(SERVER_URL))
        }

    @Test
    fun basicCrudTest() {
        val svix = getTestClient() ?: return
        runBlocking {
            val applicationOut = svix.application.create(ApplicationIn(name = "App1"))
            assertEquals("App1", applicationOut.name)
            svix.message.create(
                applicationOut.id,
                MessageIn(
                    eventType = "invoice.paid",
                    payload = kotlinx.serialization.json.Json.encodeToString(
                        mapOf<String, JsonPrimitive>(
                            "id" to JsonPrimitive("invoice_WF7WtCLFFtd8ubcTgboSFNql"),
                            "status" to JsonPrimitive("paid"),
                            "attempt" to JsonPrimitive(2),
                        )
                    ),
                ),
            )
            svix.application.delete(applicationOut.id)
        }
    }

    @Test
    fun kitchenSinkTest() {
        val svix = getTestClient() ?: return

        runBlocking {
            val appOut = svix.application.create(ApplicationIn(name = "App2"))
            try {
                svix.eventType.create(
                    EventTypeIn(name = "event.started", description = "Something started")
                )
            } catch (e: ApiException) {
                assertEquals(
                    409,
                    e.statusCode,
                    "conflicts are expected but other bad statuses are not",
                )
            }
            try {
                svix.eventType.create(
                    EventTypeIn(name = "event.ended", description = "Something ended")
                )
            } catch (e: ApiException) {
                assertEquals(
                    409,
                    e.statusCode,
                    "conflicts are expected but other bad statuses are not",
                )
            }

            val epOut =
                svix.endpoint.create(
                    appOut.id,
                    EndpointIn(url = "https://example.svix.com", channels = setOf("ch0", "ch1")),
                )
            assertEquals(setOf("ch0", "ch1"), epOut.channels, "initial ep should have 2 channels")
            assertEquals(0, epOut.eventTypes?.size ?: 0, "initial ep should have 0 filter types")
            val epPatched =
                svix.endpoint.patch(
                    appOut.id,
                    epOut.id,
                    EndpointPatch(
                        eventTypes = MaybeUnset.Present(setOf("event.started", "event.ended"))
                    ),
                )
            assertEquals(
                setOf("ch0", "ch1"),
                epPatched.channels,
                "patched ep should have 2 channels",
            )
            assertEquals(
                setOf("event.started", "event.ended"),
                epPatched.eventTypes,
                "patched ep should have 2 filter types",
            )
            svix.application.delete(appOut.id)
        }
    }

    @Test
    fun testServerUrlTrailingSlashesAreStripped() {
        val client1 = SvixHttpClient("token", "https://api.example.com/".toHttpUrlOrNull()!!, listOf(50L))
        assertEquals("https://api.example.com/", client1.baseUrl.toString())

        val client2 = SvixHttpClient("token", "https://api.example.com///".toHttpUrlOrNull()!!, listOf(50L))
        assertEquals("https://api.example.com/", client2.baseUrl.toString())

        val client3 = SvixHttpClient("token", "https://api.example.com/prefix/".toHttpUrlOrNull()!!, listOf(50L))
        assertEquals("https://api.example.com/prefix", client3.baseUrl.toString())

        val client4 = SvixHttpClient("token", "https://api.example.com/prefix///".toHttpUrlOrNull()!!, listOf(50L))
        assertEquals("https://api.example.com/prefix", client4.baseUrl.toString())

        val client5 = SvixHttpClient("token", "https://api.example.com/prefix".toHttpUrlOrNull()!!, listOf(50L))
        assertEquals("https://api.example.com/prefix", client5.baseUrl.toString())

        val client6 = SvixHttpClient("token", "https://api.example.com".toHttpUrlOrNull()!!, listOf(50L))
        assertEquals("https://api.example.com/", client6.baseUrl.toString())
    }

    @Test
    fun testDefaultAndRegionalServerUrls() {
        val defaultUrl = baseUrlFromToken("token").toHttpUrlOrNull()!!
        val defaultClient = SvixHttpClient("token", defaultUrl, listOf(50L))
        assertEquals("https://api.svix.com/", defaultClient.baseUrl.toString())

        val regionalUrl = baseUrlFromToken("testsk.eu").toHttpUrlOrNull()!!
        val regionalClient = SvixHttpClient("testsk.eu", regionalUrl, listOf(50L))
        assertEquals("https://api.eu.svix.com/", regionalClient.baseUrl.toString())
    }

    @Test
    fun testConstructedRequestUrls() {
        val defaultUrl = baseUrlFromToken("token").toHttpUrlOrNull()!!
        val defaultClient = SvixHttpClient("token", defaultUrl, listOf(50L))
        assertEquals(
            "https://api.svix.com/api/v1/app",
            defaultClient.newUrlBuilder().encodedPath("/api/v1/app").build().toString(),
        )

        val regionalUrl = baseUrlFromToken("testsk.eu").toHttpUrlOrNull()!!
        val regionalClient = SvixHttpClient("testsk.eu", regionalUrl, listOf(50L))
        assertEquals(
            "https://api.eu.svix.com/api/v1/app",
            regionalClient.newUrlBuilder().encodedPath("/api/v1/app").build().toString(),
        )

        val customClient =
            SvixHttpClient("token", "https://api.example.com/".toHttpUrlOrNull()!!, listOf(50L))
        assertEquals(
            "https://api.example.com/api/v1/app",
            customClient.newUrlBuilder().encodedPath("/api/v1/app").build().toString(),
        )

        val customMultiClient =
            SvixHttpClient("token", "https://api.example.com///".toHttpUrlOrNull()!!, listOf(50L))
        assertEquals(
            "https://api.example.com/api/v1/app",
            customMultiClient.newUrlBuilder().encodedPath("/api/v1/app").build().toString(),
        )
    }
}
