// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum Oauth2AuthMethodInOut implements ToQueryParam {
    CLIENT_SECRET_JWT("clientSecretJwt"),
    CLIENT_SECRET_BASIC("clientSecretBasic"),
    CLIENT_SECRET_POST("clientSecretPost");
    private final String value;

    Oauth2AuthMethodInOut(String value) {
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
