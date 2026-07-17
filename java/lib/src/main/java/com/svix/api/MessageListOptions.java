// this file is @generated
package com.svix.api;

import lombok.Data;

import java.time.OffsetDateTime;
import java.util.Set;

@Data
public class MessageListOptions {
    /** Limit the number of returned items */
    Long limit;

    /** The iterator returned from a prior invocation */
    String iterator;

    /** Filter response based on the channel. */
    String channel;

    /** Only include items created before a certain date. */
    OffsetDateTime before;

    /** Only include items created after a certain date. */
    OffsetDateTime after;

    /**
     * When `true` message payloads are included in the response.
     *
     * <p>Defaults to `false` in v2+ of the Svix SDKs, `true` in v1 or when manually making a
     * request without specifying this parameter.
     */
    Boolean withContent;

    /** Filter messages matching the provided tag. */
    String tag;

    /** Filter response based on the event type */
    Set<String> eventTypes;
}
