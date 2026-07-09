// this file is @generated
package com.svix.api;

import com.svix.SvixHttpClient;

import lombok.Getter;

public class Ingest {
    private final SvixHttpClient client;

    @Getter private final IngestAuthentication authentication;
    @Getter private final IngestEndpoint endpoint;
    @Getter private final IngestSource source;

    public Ingest(SvixHttpClient client) {
        this.client = client;
        this.authentication = new IngestAuthentication(client);
        this.endpoint = new IngestEndpoint(client);
        this.source = new IngestSource(client);
    }
}
