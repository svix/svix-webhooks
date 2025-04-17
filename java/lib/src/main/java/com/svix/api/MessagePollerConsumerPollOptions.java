// this file is @generated
package com.svix.api;

import lombok.Data;

@Data
public class MessagePollerConsumerPollOptions {
    /** Limit the number of returned items */
    Long limit;

    /** The iterator returned from a prior invocation */
    String iterator;
}
