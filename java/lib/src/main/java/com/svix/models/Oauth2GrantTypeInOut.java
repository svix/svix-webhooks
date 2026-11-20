// This file is @generated
package com.svix.models;

import com.fasterxml.jackson.annotation.JsonValue;
import com.svix.Utils.ToQueryParam;

public enum Oauth2GrantTypeInOut implements ToQueryParam {
    CLIENT_CREDENTIALS("clientCredentials"),
    REFRESH_TOKEN("refreshToken");
    private final String value;

    Oauth2GrantTypeInOut(String value) {
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
