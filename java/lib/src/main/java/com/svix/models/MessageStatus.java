// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum MessageStatus implements ToQueryParam {
    SUCCESS(0),
    PENDING(1),
    FAIL(2),
    SENDING(3);
    private final long value;

    MessageStatus(long value) {
        this.value = value;
    }

    @JsonValue
    public long getValue() {
        return this.value;
    }

    @Override
    public String toQueryParam() {
        return Long.toString(this.value);
    }
}
