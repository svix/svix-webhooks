package com.svix.exceptions;

public class EmptyWebhookSecretException extends Exception {

    public EmptyWebhookSecretException(final String message) {
        super(message);
    }
}
