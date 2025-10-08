// this file is @generated
package com.svix.api;

import lombok.Data;

import java.time.OffsetDateTime;

@Data
public class StreamEventsGetOptions {
    /** Limit the number of returned items */
    Long limit;

    /** The iterator returned from a prior invocation */
    String iterator;

    OffsetDateTime after;
}
