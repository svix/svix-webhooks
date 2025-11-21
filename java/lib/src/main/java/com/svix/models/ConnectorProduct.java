// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum ConnectorProduct implements ToQueryParam {
    DISPATCH("Dispatch"),
    STREAM("Stream");
    private final String value;

    ConnectorProduct(String value) {
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
