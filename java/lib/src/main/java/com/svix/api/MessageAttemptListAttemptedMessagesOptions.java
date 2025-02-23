// this file is @generated
package com.svix.api;

import com.svix.models.MessageStatus;

import lombok.Data;

import java.time.OffsetDateTime;
import java.util.Set;

@Data
public class MessageAttemptListAttemptedMessagesOptions {
    /** Limit the number of returned items */
    Long limit;

    /** The iterator returned from a prior invocation */
    String iterator;

    /** Filter response based on the channel */
    String channel;

    /** Filter response based on the message tags */
    String tag;

    /**
     * Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or
     * Sending (3)
     */
    MessageStatus status;

    /** Only include items created before a certain date */
    OffsetDateTime before;

    /** Only include items created after a certain date */
    OffsetDateTime after;

    /** When `true` message payloads are included in the response */
    Boolean withContent;

    /** Filter response based on the event type */
    Set<String> eventTypes;
}
