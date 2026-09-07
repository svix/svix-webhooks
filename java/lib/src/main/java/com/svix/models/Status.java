// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum Status implements ToQueryParam {
    PENDING("pending"),
    ACTIVE("active");
    private final String value;

    Status(String value) {
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
