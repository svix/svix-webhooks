// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum EndpointChallengeMode implements ToQueryParam {
    CHALLENGE_RESPONSE("challenge-response"),
    ECHO("echo");
    private final String value;

    EndpointChallengeMode(String value) {
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
