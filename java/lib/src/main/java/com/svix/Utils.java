package com.svix;

import com.svix.exceptions.ApiException;

final class Utils {
    public static ApiException wrapInternalApiException(com.svix.internal.ApiException e) {
        return new ApiException(e.getMessage(), e, e.getCode(), e.getResponseHeaders(), e.getResponseBody());
    }
}
