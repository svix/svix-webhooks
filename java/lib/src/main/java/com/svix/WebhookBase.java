package com.svix;

import com.svix.exceptions.WebhookSigningException;
import com.svix.exceptions.WebhookVerificationException;

import java.nio.charset.StandardCharsets;
import java.security.InvalidKeyException;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Base64;
import java.util.List;
import java.util.Map;

import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;

/**
 * Base class containing all shared webhook verification logic.
 * Used by both the Java 8 and Java 11 implementations.
 * Package-private to prevent extension outside this package.
 */
abstract class WebhookBase {
    public static final String SECRET_PREFIX = "whsec_";
    public static final String SVIX_MSG_ID_KEY = "svix-id";
    public static final String SVIX_MSG_SIGNATURE_KEY = "svix-signature";
    public static final String SVIX_MSG_TIMESTAMP_KEY = "svix-timestamp";
    public static final String UNBRANDED_MSG_ID_KEY = "webhook-id";
    public static final String UNBRANDED_MSG_SIGNATURE_KEY = "webhook-signature";
    public static final String UNBRANDED_MSG_TIMESTAMP_KEY = "webhook-timestamp";
    private static final String HMAC_SHA256 = "HmacSHA256";
    private static final int TOLERANCE_IN_SECONDS = 5 * 60; // 5 minutes
    private static final long SECOND_IN_MS = 1000L;

    protected final byte[] key;

    protected WebhookBase(final String secret) {
        String sec = secret;
        if (sec.startsWith(WebhookBase.SECRET_PREFIX)) {
            sec = sec.substring(WebhookBase.SECRET_PREFIX.length());
        }
        this.key = Base64.getDecoder().decode(sec);
    }

    protected WebhookBase(final byte[] secret) {
        this.key = secret;
    }

    public void verify(final String payload, final Map<String, List<String>> headers)
            throws WebhookVerificationException {
        String msgId = firstHeader(headers, SVIX_MSG_ID_KEY);
        String msgSignature = firstHeader(headers, SVIX_MSG_SIGNATURE_KEY);
        String msgTimestamp = firstHeader(headers, SVIX_MSG_TIMESTAMP_KEY);

        if (msgId == null || msgSignature == null || msgTimestamp == null) {
            // fallback to unbranded
            msgId = firstHeader(headers, UNBRANDED_MSG_ID_KEY);
            msgSignature = firstHeader(headers, UNBRANDED_MSG_SIGNATURE_KEY);
            msgTimestamp = firstHeader(headers, UNBRANDED_MSG_TIMESTAMP_KEY);
            if (msgId == null || msgSignature == null || msgTimestamp == null) {
                throw new WebhookVerificationException("Missing required headers");
            }
        }

        long timestamp = WebhookBase.verifyTimestamp(msgTimestamp);

        String expectedSignature;
        try {
            expectedSignature = this.sign(msgId, timestamp, payload).split(",")[1];
        } catch (WebhookSigningException e) {
            throw new WebhookVerificationException("Failed to generate expected signature");
        }

        String[] msgSignatures = msgSignature.split(" ");
        for (String versionedSignature : msgSignatures) {
            String[] sigParts = versionedSignature.split(",");
            if (sigParts.length < 2) {
                continue;
            }
            String version = sigParts[0];
            if (!version.equals("v1")) {
                continue;
            }
            String signature = sigParts[1];
            if (MessageDigest.isEqual(signature.getBytes(), expectedSignature.getBytes())) {
                return;
            }
        }
        throw new WebhookVerificationException("No matching signature found");
    }

    private static String firstHeader(
            final Map<String, List<String>> headers, final String name) {
        for (Map.Entry<String, List<String>> entry : headers.entrySet()) {
            if (entry.getKey().equalsIgnoreCase(name)) {
                List<String> values = entry.getValue();
                if (values != null && !values.isEmpty()) {
                    return values.get(0);
                }
            }
        }
        return null;
    }

    public String sign(final String msgId, final long timestamp, final String payload)
            throws WebhookSigningException {
        try {
            String toSign = String.format("%s.%s.%s", msgId, timestamp, payload);
            Mac sha512Hmac = Mac.getInstance(HMAC_SHA256);
            SecretKeySpec keySpec = new SecretKeySpec(this.key, HMAC_SHA256);
            sha512Hmac.init(keySpec);
            byte[] macData = sha512Hmac.doFinal(toSign.getBytes(StandardCharsets.UTF_8));
            String signature = Base64.getEncoder().encodeToString(macData);
            return String.format("v1,%s", signature);
        } catch (InvalidKeyException | NoSuchAlgorithmException e) {
            throw new WebhookSigningException(e.getMessage());
        }
    }

    private static long verifyTimestamp(final String timestampHeader)
            throws WebhookVerificationException {
        long now = System.currentTimeMillis() / WebhookBase.SECOND_IN_MS;

        long timestamp;
        try {
            timestamp = Long.parseLong(timestampHeader);
        } catch (NumberFormatException e) {
            throw new WebhookVerificationException("Invalid Signature Headers");
        }

        if (timestamp < (now - TOLERANCE_IN_SECONDS)) {
            throw new WebhookVerificationException("Message timestamp too old");
        }
        if (timestamp > (now + TOLERANCE_IN_SECONDS)) {
            throw new WebhookVerificationException("Message timestamp too new");
        }
        return timestamp;
    }
}
