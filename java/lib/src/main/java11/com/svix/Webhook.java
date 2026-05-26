package com.svix;

import com.svix.exceptions.WebhookVerificationException;

import java.net.http.HttpHeaders;

/**
 * A class for verifying and generating webhook signatures.
 */
public final class Webhook extends WebhookBase {

    public Webhook(final String secret) throws EmptyWebhookSecretException {
        super(secret);
    }

    public Webhook(final byte[] secret) throws EmptyWebhookSecretException {
        super(secret);
    }

    public void verify(final String payload, final HttpHeaders headers)
            throws WebhookVerificationException {
        verify(payload, headers.map());
    }
}
