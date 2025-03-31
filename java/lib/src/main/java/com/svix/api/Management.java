// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;

import lombok.Getter;

public class Management {
    private final SvixHttpClient client;

    @Getter private final ManagementAuthentication authentication;

    public Management(SvixHttpClient client) {
        this.client = client;
        this.authentication = new ManagementAuthentication(client);
    }
}
