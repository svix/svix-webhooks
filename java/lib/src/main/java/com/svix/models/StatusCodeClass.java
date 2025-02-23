// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum StatusCodeClass implements ToQueryParam {
    CODE_NONE(0),
    CODE1XX(100),
    CODE2XX(200),
    CODE3XX(300),
    CODE4XX(400),
    CODE5XX(500);
    private final long value;

    StatusCodeClass(long value) {
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
