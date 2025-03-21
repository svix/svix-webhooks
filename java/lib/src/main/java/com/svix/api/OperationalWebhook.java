// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;

import lombok.Getter;

public class OperationalWebhook {
    private final SvixHttpClient client;

    @Getter private final OperationalWebhookEndpoint endpoint;

    public OperationalWebhook(SvixHttpClient client) {
        this.client = client;
        this.endpoint = new OperationalWebhookEndpoint(client);
    }
}
