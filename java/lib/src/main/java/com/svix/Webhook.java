package com.svix;

import com.svix.exceptions.EmptyWebhookSecretException;

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
}
