package com.svix.exceptions;

import java.util.Map;
import java.util.List;
import com.google.gson.Gson;

class APIError {
    private String code;
    private String detail;

    public String getCode() {
        return this.code;
    }

    public String getDetail() {
        return this.detail;
    }
}

public class ApiException extends com.svix.internal.ApiException {

    private String message;

    public ApiException(final String message, final Throwable throwable, final int code, final Map<String, List<String>> responseHeaders, final String responseBody) {
        super(message, throwable, code, responseHeaders, responseBody);

        if ((message == null || message.isEmpty()) && responseBody != null) {
            try {
                Gson gson = new Gson();
                APIError error = gson.fromJson(responseBody, APIError.class);
                this.message = String.format("%s (%s)", error.getDetail(), error.getCode());
            } catch (Exception e) {
                this.message = responseBody;
            }
        }
    }

    public String getMessage() {
        String msg = super.getMessage();
        if (msg != null && !msg.isEmpty()) {
            return msg;
        }
        return this.message;
    }
}
