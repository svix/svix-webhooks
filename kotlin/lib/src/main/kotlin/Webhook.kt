package com.svix.kotlin

import com.svix.kotlin.exceptions.WebhookSigningException
import com.svix.kotlin.exceptions.WebhookVerificationException
import java.net.http.HttpHeaders
import java.nio.charset.StandardCharsets
import java.security.InvalidKeyException
import java.security.MessageDigest
import java.security.NoSuchAlgorithmException
import java.util.Base64
import javax.crypto.Mac
import javax.crypto.spec.SecretKeySpec

class Webhook {
    private val key: ByteArray

    @Throws(WebhookVerificationException::class)
    fun verify(
        payload: String?,
        headers: HttpHeaders,
    ) {
        var msgId = headers.firstValue(SVIX_MSG_ID_KEY)
        var msgSignature = headers.firstValue(SVIX_MSG_SIGNATURE_KEY)
        var msgTimestamp = headers.firstValue(SVIX_MSG_TIMESTAMP_KEY)
        if (msgId.isEmpty || msgSignature.isEmpty || msgTimestamp.isEmpty) {
            // fallback to unbranded
            msgId = headers.firstValue(UNBRANDED_MSG_ID_KEY)
            msgSignature = headers.firstValue(UNBRANDED_MSG_SIGNATURE_KEY)
            msgTimestamp = headers.firstValue(UNBRANDED_MSG_TIMESTAMP_KEY)
            if (msgId.isEmpty || msgSignature.isEmpty || msgTimestamp.isEmpty) {
                throw WebhookVerificationException("Missing required headers")
            }
        }
        val timestamp: Long = verifyTimestamp(msgTimestamp.get())
        val expectedSignature: String =
            try {
                sign(msgId.get(), timestamp, payload).split(",".toRegex()).toTypedArray()[1]
            } catch (e: WebhookSigningException) {
                throw WebhookVerificationException("Failed to generate expected signature")
            }
        val msgSignatures = msgSignature.get().split(" ".toRegex()).toTypedArray()
        for (versionedSignature in msgSignatures) {
            val sigParts = versionedSignature.split(",".toRegex()).toTypedArray()
            if (sigParts.size < 2) {
                continue
            }
            val version = sigParts[0]
            if (version != "v1") {
                continue
            }
            val signature = sigParts[1]
            if (MessageDigest.isEqual(signature.toByteArray(), expectedSignature.toByteArray())) {
                return
            }
        }
        throw WebhookVerificationException("No matching signature found")
    }

    @Throws(WebhookSigningException::class)
    fun sign(
        msgId: String?,
        timestamp: Long,
        payload: String?,
    ): String {
        return try {
            val toSign = String.format("%s.%s.%s", msgId, timestamp, payload)
            val sha512Hmac: Mac = Mac.getInstance(HMAC_SHA256)
            val keySpec = SecretKeySpec(key, HMAC_SHA256)
            sha512Hmac.init(keySpec)
            val macData: ByteArray = sha512Hmac.doFinal(toSign.toByteArray(StandardCharsets.UTF_8))
            val signature = Base64.getEncoder().encodeToString(macData)
            String.format("v1,%s", signature)
        } catch (e: InvalidKeyException) {
            throw WebhookSigningException(e)
        } catch (e: NoSuchAlgorithmException) {
            throw WebhookSigningException(e)
        }
    }

    companion object {
        const val SECRET_PREFIX = "whsec_"
        const val SVIX_MSG_ID_KEY = "svix-id"
        const val SVIX_MSG_SIGNATURE_KEY = "svix-signature"
        const val SVIX_MSG_TIMESTAMP_KEY = "svix-timestamp"
        const val UNBRANDED_MSG_ID_KEY = "webhook-id"
        const val UNBRANDED_MSG_SIGNATURE_KEY = "webhook-signature"
        const val UNBRANDED_MSG_TIMESTAMP_KEY = "webhook-timestamp"
        private const val HMAC_SHA256 = "HmacSHA256"
        private const val TOLERANCE_IN_SECONDS = 5 * 60 // 5 minutes
        private const val SECOND_IN_MS = 1000L

        @Throws(WebhookVerificationException::class)
        private fun verifyTimestamp(timestampHeader: String): Long {
            val now: Long = System.currentTimeMillis() / SECOND_IN_MS
            val timestamp: Long =
                try {
                    timestampHeader.toLong()
                } catch (e: NumberFormatException) {
                    throw WebhookVerificationException("Invalid Signature Headers")
                }

            if (timestamp < now - TOLERANCE_IN_SECONDS) {
                throw WebhookVerificationException("Message timestamp too old")
            }
            if (timestamp > now + TOLERANCE_IN_SECONDS) {
                throw WebhookVerificationException("Message timestamp too new")
            }
            return timestamp
        }
    }

    constructor(secret: String) {
        var sec = secret
        if (sec.startsWith(SECRET_PREFIX)) {
            sec = sec.substring(SECRET_PREFIX.length)
        }
        key = Base64.getDecoder().decode(sec)
    }

    constructor(secret: ByteArray) {
        key = secret
    }
}
