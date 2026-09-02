package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.MessagePollerv2ConsumerPollOptions
import com.svix.kotlin.models.EndpointIn
import com.svix.kotlin.models.SinkInCommon
import com.svix.kotlin.models.StartingPosition
import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse
import kotlin.test.Test
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

class AcItHarness {
    @Serializable
    private data class LangFixtures(
        val appId: String,
        val v1Http: String,
        val v1Poller: String,
        val v2Http: String,
        val v2Poller: String,
        val v2PollerExisting: String,
    )

    @Serializable
    private data class FixturesFile(
        val serverUrl: String,
        val orgToken: String,
        val eventType: String,
        val httpUrl: String,
        val consumerId: String,
        val languages: Map<String, LangFixtures>,
    )

    @Test
    fun liveAutoConfig() {
        val json = Json { ignoreUnknownKeys = true }
        val fixturesPath = System.getenv("AUTOCONFIG_FIXTURES")
            ?: error("AUTOCONFIG_FIXTURES is unset")
        val fixtures =
            json.decodeFromString<FixturesFile>(java.io.File(fixturesPath).readText())
        val lang = fixtures.languages["kotlin"]!!
        val results = LinkedHashMap<String, String>()
        val endpointIn =
            EndpointIn(url = fixtures.httpUrl, eventTypes = setOf(fixtures.eventType))

        results["A"] =
            runCase("v1 HTTP") {
                val out = runBlocking { AutoConfig(lang.v1Http, endpointIn).subscribe() }
                check(out.url == fixtures.httpUrl) {
                    "url mismatch: ${out.url} != ${fixtures.httpUrl}"
                }
                "id=${out.id} url=${out.url}"
            }

        results["B"] =
            runCase("v2 HTTP") {
                val out = runBlocking { AutoConfig(lang.v2Http, endpointIn).subscribe() }
                check(out.url == fixtures.httpUrl) {
                    "url mismatch: ${out.url} != ${fixtures.httpUrl}"
                }
                "id=${out.id} url=${out.url}"
            }

        results["C"] =
            runCase("v1 poller") {
                runPoller(
                    token = lang.v1Poller,
                    src = "kotlin-v1",
                    fixtures = fixtures,
                    appId = lang.appId,
                )
            }

        results["D"] =
            runCase("v2 poller") {
                runPoller(
                    token = lang.v2Poller,
                    src = "kotlin-v2",
                    fixtures = fixtures,
                    appId = lang.appId,
                )
            }

        results["E"] =
            runCase("v2 receive without subscribe") {
                runPollerExisting(
                    token = lang.v2PollerExisting,
                    src = "kotlin-e",
                    fixtures = fixtures,
                    appId = lang.appId,
                )
            }

        val report =
            buildString {
                appendLine("LANG: kotlin")
                for (key in listOf("A", "B", "C", "D", "E")) {
                    appendLine("$key: ${results[key]}")
                }
                appendLine(
                    "Notes: copied into kotlin test source set for this run; JDK 17"
                )
            }
        val outPath = System.getenv("AUTOCONFIG_RESULTS")
        if (outPath != null) {
            java.io.File(outPath).writeText(report)
        }
        print(report)
    }

    private fun runCase(label: String, block: () -> String): String {
        return try {
            val detail = block()
            "PASS — $label $detail"
        } catch (e: ApiException) {
            val body = e.body?.replace('\n', ' ')?.take(240) ?: ""
            "FAIL — $label ${e.message} $body".trim()
        } catch (e: Throwable) {
            val msg = e.message?.replace('\n', ' ') ?: e.javaClass.simpleName
            "FAIL — $label $msg"
        }
    }

    private fun runPoller(
        token: String,
        src: String,
        fixtures: FixturesFile,
        appId: String,
    ): String {
        return runBlocking {
            val consumer =
                AutoConfigConsumer(
                    token,
                    SinkInCommon(eventTypes = setOf(fixtures.eventType)),
                )
            val dest = consumer.subscribe()
            sendMsg(fixtures, appId, src)

            val options =
                MessagePollerv2ConsumerPollOptions(
                    startingPosition = StartingPosition.EARLIEST,
                    leaseDurationMs = 2000UL,
                )

            var foundOffset: ULong? = null
            var lastCount = 0
            for (attempt in 1..10) {
                val poll = consumer.receive(fixtures.consumerId, options)
                lastCount = poll.data.size
                val match =
                    poll.data.firstOrNull { msg ->
                        msg.eventType == fixtures.eventType &&
                            (msg.payload["src"]?.toString() == src)
                    }
                if (match != null) {
                    foundOffset = match.offset
                    break
                }
                delay(2000)
            }
            check(foundOffset != null) {
                "no message with src=$src after 10 retries (last poll size=$lastCount)"
            }

            consumer.commit(fixtures.consumerId, foundOffset!!)

            val after = consumer.receive(fixtures.consumerId, options)
            val replayed = after.data.any { it.offset == foundOffset }
            check(!replayed) { "committed offset $foundOffset was replayed" }

            "dest=${dest.id} offset=$foundOffset"
        }
    }

    private fun runPollerExisting(
        token: String,
        src: String,
        fixtures: FixturesFile,
        appId: String,
    ): String {
        return runBlocking {
            val binder =
                AutoConfigConsumer(
                    token,
                    SinkInCommon(eventTypes = setOf(fixtures.eventType)),
                )
            val dest = binder.subscribe()
            val consumer =
                AutoConfigConsumer(
                    token,
                    SinkInCommon(eventTypes = setOf(fixtures.eventType)),
                )
            sendMsg(fixtures, appId, src)

            val options =
                MessagePollerv2ConsumerPollOptions(
                    startingPosition = StartingPosition.EARLIEST,
                    leaseDurationMs = 2000UL,
                )

            var foundOffset: ULong? = null
            var lastCount = 0
            for (attempt in 1..10) {
                val poll = consumer.receive(fixtures.consumerId, options)
                lastCount = poll.data.size
                val match =
                    poll.data.firstOrNull { msg ->
                        msg.eventType == fixtures.eventType &&
                            (msg.payload["src"]?.toString() == src)
                    }
                if (match != null) {
                    foundOffset = match.offset
                    break
                }
                delay(2000)
            }
            check(foundOffset != null) {
                "no message with src=$src after 10 retries (last poll size=$lastCount)"
            }

            consumer.commit(fixtures.consumerId, foundOffset!!)

            val after = consumer.receive(fixtures.consumerId, options)
            val replayed = after.data.any { it.offset == foundOffset }
            check(!replayed) { "committed offset $foundOffset was replayed" }

            "dest=${dest.id} offset=$foundOffset no subscribe on receiver"
        }
    }

    private fun sendMsg(fixtures: FixturesFile, appId: String, src: String) {
        val body =
            """{"eventType":"${fixtures.eventType}","payload":{"ok":true,"src":"$src"}}"""
        val req =
            HttpRequest.newBuilder()
                .uri(URI.create("${fixtures.serverUrl}/api/v1/app/$appId/msg/"))
                .header("Authorization", "Bearer ${fixtures.orgToken}")
                .header("Content-Type", "application/json")
                .POST(HttpRequest.BodyPublishers.ofString(body))
                .build()
        val resp = HttpClient.newHttpClient().send(req, HttpResponse.BodyHandlers.ofString())
        check(resp.statusCode() in 200..299) {
            "send msg status=${resp.statusCode()} body=${resp.body()}"
        }
    }
}
