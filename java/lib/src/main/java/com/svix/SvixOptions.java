package com.svix;

import lombok.Getter;
import lombok.Setter;

import java.util.Arrays;
import java.util.List;

@Setter
@Getter
public final class SvixOptions {
    public static final String DEFAULT_URL = "https://api.svix.com";

    private String serverUrl;
    private final List<Long> retrySchedule = Arrays.asList(50L, 100L, 200L);
    private boolean debug = false;
}
