// this file is @generated
package com.svix.api;

import lombok.Data;

import java.time.OffsetDateTime;

@Data
public class EndpointGetStatsOptions {
    /** Filter the range to data starting from this date. */
    OffsetDateTime since;

    /** Filter the range to data ending by this date. */
    OffsetDateTime until;
}
