// this file is @generated
package com.svix.internalapi;

import com.svix.SvixHttpClient;

import lombok.Getter;

public class Destination {
    private final SvixHttpClient client;

    @Getter private final DestinationAutoconfig autoconfig;

    public Destination(SvixHttpClient client) {
        this.client = client;
        this.autoconfig = new DestinationAutoconfig(client);
    }
}
