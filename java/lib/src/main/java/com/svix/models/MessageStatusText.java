// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum MessageStatusText implements ToQueryParam {
    SUCCESS("success"),
    PENDING("pending"),
    FAIL("fail"),
    SENDING("sending");
    private final String value;

    MessageStatusText(String value) {
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
