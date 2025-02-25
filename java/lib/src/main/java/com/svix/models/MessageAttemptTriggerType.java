// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum MessageAttemptTriggerType implements ToQueryParam {
    SCHEDULED(0),
    MANUAL(1);
    private final long value;

    MessageAttemptTriggerType(long value) {
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
