package com.svix;

import lombok.Getter;
import lombok.Setter;

import java.util.Arrays;
import java.util.List;

@Setter
@Getter
public final class SvixOptions {
    public static final String DEFAULT_URL = "https://api.svix.com";

    /**
     * Override the default API URL.
     */
    private String serverUrl;

    /**
     * Delay, in milliseconds, to wait before each retry of a failed request.
     *
     * <p>The list has one entry per retry. Defaults to {@code [50, 100, 200]}.
     */
    private List<Long> retrySchedule = Arrays.asList(50L, 100L, 200L);

    private boolean debug = false;
}
