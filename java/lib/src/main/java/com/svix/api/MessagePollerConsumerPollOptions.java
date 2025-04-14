// this file is @generated
package com.svix.api;

import lombok.Data;

@Data
public class MessagePollerConsumerPollOptions {
    /** Limit the number of returned items */
    Long limit;

    /** The iterator returned from a prior invocation */
    String iterator;

    /** Filters messages sent with this event type (optional). */
    String eventType;

    /** Filters messages sent with this channel (optional). */
    String channel;
}
