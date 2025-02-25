// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum BackgroundTaskStatus implements ToQueryParam {
    RUNNING("running"),
    FINISHED("finished"),
    FAILED("failed");
    private final String value;

    BackgroundTaskStatus(String value) {
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
