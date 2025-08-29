// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum AppPortalCapability implements ToQueryParam {
    VIEW_BASE("ViewBase"),
    VIEW_ENDPOINT_SECRET("ViewEndpointSecret"),
    MANAGE_ENDPOINT_SECRET("ManageEndpointSecret"),
    MANAGE_TRANSFORMATIONS("ManageTransformations"),
    CREATE_ATTEMPTS("CreateAttempts"),
    MANAGE_ENDPOINT("ManageEndpoint");
    private final String value;

    AppPortalCapability(String value) {
        this.value = value;
    }

    @JsonValue
    public String getValue() {
        return this.value;
    }

    @Override
    public String toQueryParam() {
        return this.value;
    }
}
