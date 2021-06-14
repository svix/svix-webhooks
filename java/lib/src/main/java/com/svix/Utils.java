package com.svix;

import com.svix.exceptions.ApiException;

final class Utils {
    private Utils() {
    }

    public static ApiException wrapInternalApiException(final com.svix.internal.ApiException e) {
        return new ApiException(e.getMessage(), e, e.getCode(), e.getResponseHeaders(), e.getResponseBody());
    }
}
