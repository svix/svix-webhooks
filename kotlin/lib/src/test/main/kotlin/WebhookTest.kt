package com.svix.kotlin

import com.svix.kotlin.exceptions.WebhookSigningException
import com.svix.kotlin.exceptions.WebhookVerificationException
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import java.net.http.HttpHeaders
import java.nio.charset.StandardCharsets
import java.util.Arrays
import java.util.Base64
import java.util.function.BiPredicate
import javax.crypto.Mac
import javax.crypto.spec.SecretKeySpec
import kotlin.collections.ArrayList
import kotlin.collections.HashMap
import kotlin.test.Test

class WebhookTest {
    @Test
    @Throws(WebhookVerificationException::class)
    fun verifyValidPayloadAndheader() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis())
        val webhook = Webhook(testPayload.secret)
        webhook.verify(testPayload.payload, testPayload.headers())
    }

    @Test
    @Throws(WebhookVerificationException::class)
    fun verifyValidBrandlessPayloadAndheader() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap["webhook-id"] = testPayload.headerMap["svix-id"]!!
        testPayload.headerMap["webhook-timestamp"] = testPayload.headerMap["svix-timestamp"]!!
        testPayload.headerMap["webhook-signature"] = testPayload.headerMap["svix-signature"]!!
        val webhook = Webhook(testPayload.secret)
        webhook.verify(testPayload.payload, testPayload.headers())
    }

    @Test
    @Throws(WebhookVerificationException::class)
    fun verifyValidPayloadWithMultipleSignaturesIsValid() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis())
        val sigs = arrayOf(
            "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
            "v2,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc=",
            testPayload.headerMap["svix-signature"]!![0],
            "v1,Ceo5qEr07ixe2NLpvHk3FH9bwy/WavXrAFQ/9tdO6mc="
        )
        testPayload.headerMap["svix-signature"] = ArrayList<String>(Arrays.asList<String>(java.lang.String.join(" ", *sigs)))
        val webhook = Webhook(testPayload.secret)
        webhook.verify(testPayload.payload, testPayload.headers())
    }

    @Test
    fun verifyMissingIdThrowsException() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap.remove("svix-id")
        val webhook = Webhook(testPayload.secret)
        assertThrows(WebhookVerificationException::class.java) {
            webhook.verify(testPayload.payload, testPayload.headers())
        }
    }

    @Test
    fun verifyMissingTimestampThrowsException() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap.remove("svix-timestamp")
        val webhook = Webhook(testPayload.secret)
        assertThrows(WebhookVerificationException::class.java) {
            webhook.verify(testPayload.payload, testPayload.headers())
        }
    }

    @Test
    fun verifyMissingSignatureThrowsException() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap.remove("svix-signature")
        val webhook = Webhook(testPayload.secret)
        assertThrows(WebhookVerificationException::class.java) {
            webhook.verify(testPayload.payload, testPayload.headers())
        }
    }

    @Test
    fun verifySignatureWithDifferentVersionThrowsException() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap[Webhook.SVIX_MSG_ID_KEY] = ArrayList<String>(Arrays.asList<String>("v2,g0hM9SsE+OTPJTGt/tmIKtSyZlE3uFJELVlNIOLJ1OE="))
        val webhook = Webhook(testPayload.secret)
        assertThrows(WebhookVerificationException::class.java) {
            webhook.verify(testPayload.payload, testPayload.headers())
        }
    }

    @Test
    fun verifyMissingPartsInSignatureThrowsException() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap[Webhook.SVIX_MSG_ID_KEY] = ArrayList<String>(Arrays.asList<String>("invalid_signature"))
        val webhook = Webhook(testPayload.secret)
        assertThrows(WebhookVerificationException::class.java) {
            webhook.verify(testPayload.payload, testPayload.headers())
        }
    }

    @Test
    fun verifySignatureMismatchThrowsException() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis())
        testPayload.headerMap[Webhook.SVIX_MSG_ID_KEY] = ArrayList<String>(Arrays.asList<String>("v1,invalid_signature"))
        val webhook = Webhook(testPayload.secret)
        assertThrows(WebhookVerificationException::class.java) {
            webhook.verify(testPayload.payload, testPayload.headers())
        }
    }

    @Test
    fun verifyOldTimestampThrowsException() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis() + TOLERANCE_IN_MS + SECOND_IN_MS)
        val webhook = Webhook(testPayload.secret)
        assertThrows(WebhookVerificationException::class.java) {
            webhook.verify(testPayload.payload, testPayload.headers())
        }
    }

    @Test
    fun verifyNewTimestampThrowsException() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis() - TOLERANCE_IN_MS - SECOND_IN_MS)
        val webhook = Webhook(testPayload.secret)
        assertThrows(WebhookVerificationException::class.java) {
            webhook.verify(testPayload.payload, testPayload.headers())
        }
    }

    @Test
    @Throws(WebhookVerificationException::class)
    fun verifySecretWorksWithOrWithoutPrefix() {
        val testPayload: TestPayload = TestPayload(System.currentTimeMillis())
        var webhook = Webhook(testPayload.secret)
        webhook.verify(testPayload.payload, testPayload.headers())
        webhook = Webhook(String.format("%s%s", Webhook.SECRET_PREFIX, testPayload.secret))
        webhook.verify(testPayload.payload, testPayload.headers())
    }

    @Test
    @Throws(WebhookSigningException::class)
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

    private inner class TestPayload internal constructor(timestampInMS: Long) {
        private val id: String = "msg_p5jXN8AQM9LWM0D4loKWxJek"
        private val timestamp: String
        val payload: String
        val secret: String
        private var signature: String? = null
        val headerMap: HashMap<String, ArrayList<String>>
        fun headers(): HttpHeaders {
            val map = HashMap<String, List<String>>()
            for ((key, value) in headerMap) {
                map[key] = value
            }
            return HttpHeaders.of(map, BiPredicate { _, _ -> true })
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
            headerMap["svix-id"] = ArrayList<String>(Arrays.asList<String>(id))
            headerMap["svix-timestamp"] = ArrayList<String>(Arrays.asList<String>(timestamp))
            headerMap["svix-signature"] = ArrayList<String>(Arrays.asList<String>(String.format("v1,%s", signature)))
        }
    }

    companion object {
        private const val TOLERANCE_IN_MS = 5 * 60 * 1000
        private const val SECOND_IN_MS = 1000
    }
}
