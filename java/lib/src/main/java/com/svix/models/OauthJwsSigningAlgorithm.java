// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum OauthJwsSigningAlgorithm implements ToQueryParam {
    RS256("RS256");
    private final String value;

    OauthJwsSigningAlgorithm(String value) {
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
