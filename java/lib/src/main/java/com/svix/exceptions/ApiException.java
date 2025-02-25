package com.svix.exceptions;

import lombok.Getter;

@Getter
public class ApiException extends Exception {
    String message;
    String responseBody;
    int code;

    public ApiException(final String message, final int code, final String responseBody) {
        this.message = message;
        this.code = code;
        this.responseBody = responseBody;
    }
}
