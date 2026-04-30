// this file is @generated
package com.svix.api;

import lombok.Data;

@Data
public class MessageAttemptGetOptions {
    /**
     * When `true`, return the Canceled (4) status in attempts. If `false`, canceled attempts are
     * returned as Success (0)
     */
    Boolean expandedStatuses;
}
