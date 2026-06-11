// this file is @generated
package com.svix.internalapi;

import com.svix.models.StartingPosition;

import lombok.Data;

@Data
public class MessagePollerv2ConsumerPollOptions {
    Long limit;

    /** Lease duration in milliseconds. */
    Long leaseDurationMs;

    StartingPosition startingPosition;
}
