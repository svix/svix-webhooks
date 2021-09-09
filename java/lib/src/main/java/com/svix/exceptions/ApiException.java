package com.svix.exceptions;

import java.util.Map;
import java.util.List;

public class ApiException extends com.svix.internal.ApiException {
    public ApiException(final String message, final Throwable throwable, final int code, final Map<String, List<String>> responseHeaders, final String responseBody) {
        super(message, throwable, code, responseHeaders, responseBody);
    }

    public String getMessage() {
        String msg = super.getMessage();
        if (msg == "") {
            msg = this.getResponseBody();
        }
        return msg;
    }
}
