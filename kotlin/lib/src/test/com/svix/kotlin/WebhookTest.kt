package com.svix.kotlin

import com.svix.kotlin.exceptions.WebhookVerificationException
import java.net.http.HttpHeaders
import java.nio.charset.StandardCharsets
import java.util.Base64
import javax.crypto.Mac
import javax.crypto.spec.SecretKeySpec
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class WebhookTest {
    @Test
    fun verifyValidPayloadAndheader() {
        val testPayload = TestPayload(System.currentTimeMillis())
        val webhook = Webhook(testPayload.secret)
        webhook.verify(testPayload.payload, testPayload.headers())
    }

    @Test
    fun verifyValidBrandlessPayloadAndheader() {
        val testPayload = TestPayload(System.currentTimeMillis())
        val unbrandedHeaders = HashMap<String, java.util.ArrayList<String>>()
        unbrandedHeaders["webhook-id"] = testPayload.headerMap["svix-id"]!!
        unbrandedHeaders["webhook-timestamp"] = testPayload.headerMap["svix-timestamp"]!!
        unbrandedHeaders["webhook-signature"] = testPayload.headerMap["svix-signature"]!!
        testPayload.headerMap = unbrandedHeaders
        val webhook = Webhook(testPayload.secret)
        webhook.verify(testPayload.payload, testPayload.headers())
    }

    @Test
    fun verifyValidPayloadWithMultipleSignaturesIsValid() {
        val testPayload = TestPayload(System.currentTimeMillis())
        val sigs =
            arrayOf(
                "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
                "v2,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
                // valid signature
                testPayload.headerMap["svix-signature"]!![0],
                "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
            )
        testPayload.headerMap["svix-signature"] = ArrayList(listOf(sigs.joinToString(" ")))
        val webhook = Webhook(testPayload.secret)
        webhook.verify(testPayload.payload, testPayload.headers())
    }

    @Test
    fun verifyMissingIdThrowsException() {
        val testPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap.remove("svix-id")
        assertFailsWith<WebhookVerificationException>(
            block = {
                verify(testPayload)
            },
        )
    }

    @Test
    fun verifyMissingTimestampThrowsException() {
        val testPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap.remove("svix-timestamp")
        assertFailsWith<WebhookVerificationException>(
            block = {
                verify(testPayload)
            },
        )
    }

    @Test
    fun verifyMissingSignatureThrowsException() {
        val testPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap.remove("svix-signature")
        assertFailsWith<WebhookVerificationException>(
            block = {
                verify(testPayload)
            },
        )
    }

    @Test
    fun verifySignatureWithDifferentVersionThrowsException() {
        val testPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap[Webhook.SVIX_MSG_ID_KEY] = ArrayList(listOf("v2,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="))
        assertFailsWith<WebhookVerificationException>(
            block = {
                verify(testPayload)
            },
        )
    }

    @Test
    fun verifyMissingPartsInSignatureThrowsException() {
        val testPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap[Webhook.SVIX_MSG_ID_KEY] = ArrayList(listOf("invalid_signature"))
        assertFailsWith<WebhookVerificationException>(
            block = {
                verify(testPayload)
            },
        )
    }

    @Test
    fun verifySignatureMismatchThrowsException() {
        val testPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap[Webhook.SVIX_MSG_ID_KEY] = ArrayList(listOf("v1,invalid_signature"))
        assertFailsWith<WebhookVerificationException>(
            block = {
                verify(testPayload)
            },
        )
    }

    @Test
    fun verifyOldTimestampThrowsException() {
        val testPayload = TestPayload(System.currentTimeMillis() + TOLERANCE_IN_MS + SECOND_IN_MS)
        assertFailsWith<WebhookVerificationException>(
            block = {
                verify(testPayload)
            },
        )
    }

    @Test
    fun verifyNewTimestampThrowsException() {
        val testPayload = TestPayload(System.currentTimeMillis() - TOLERANCE_IN_MS - SECOND_IN_MS)
        assertFailsWith<WebhookVerificationException>(
            block = {
                verify(testPayload)
            },
        )
    }

    @Test
    fun verifySecretWorksWithOrWithoutPrefix() {
        val testPayload = TestPayload(System.currentTimeMillis())
        var webhook = Webhook(testPayload.secret)
        webhook.verify(testPayload.payload, testPayload.headers())
        webhook = Webhook(java.lang.String.format("%s%s", Webhook.SECRET_PREFIX, testPayload.secret))
        webhook.verify(testPayload.payload, testPayload.headers())
    }

    @Test
    fun verifyWebhookSignWorks() {
        val key = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw"
        val msgId = "msg_p5jXN8AQM9LWM0D4loKWxJek"
        val timestamp: Long = 1614265330
        val payload = "{\"test\": 2432232314}"
        val expected = "v1,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="
        val webhook = Webhook(key)
        val signature = webhook.sign(msgId, timestamp, payload)
        assertEquals(signature, expected)
    }

    private fun verify(testPayload: TestPayload) {
        val webhook = Webhook(testPayload.secret)
        webhook.verify(testPayload.payload, testPayload.headers())
    }

    private inner class TestPayload(timestampInMS: Long) {
        private val id: String = "msg_p5jXN8AQM9LWM0D4loKWxJek"
        private val timestamp: String
        val payload: String
        val secret: String
        private var signature: String? = null
        var headerMap: HashMap<String, ArrayList<String>>

        fun headers(): HttpHeaders {
            val map = HashMap<String, List<String>>()
            for ((key, value) in headerMap) {
                map[key] = value
            }
            return HttpHeaders.of(map) { _, _ -> true }
        }

        init {
            timestamp = (timestampInMS / SECOND_IN_MS).toString()
            payload = "{\"test\": 2432232314}"
            secret = "MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw"
            try {
                val toSign = String.format("%s.%s.%s", id, timestamp, payload)
                val sha512Hmac: Mac = Mac.getInstance("HmacSHA256")
                val keySpec = SecretKeySpec(Base64.getDecoder().decode(secret), "HmacSHA256")
                sha512Hmac.init(keySpec)
                val macData: ByteArray = sha512Hmac.doFinal(toSign.toByteArray(StandardCharsets.UTF_8))
                signature = Base64.getEncoder().encodeToString(macData)
            } catch (e: Exception) {
                // pass
            }
            headerMap = HashMap()
            headerMap["svix-id"] = ArrayList(listOf(id))
            headerMap["svix-timestamp"] = ArrayList(listOf(timestamp))
            headerMap["svix-signature"] = ArrayList(listOf(String.format("v1,%s", signature)))
        }
    }

    companion object {
        private const val TOLERANCE_IN_MS = 5 * 60 * 1000
        private const val SECOND_IN_MS = 1000
    }
}
