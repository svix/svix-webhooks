// this file is @generated
package com.svix.api;

import com.svix.models.Ordering;

import lombok.Data;

@Data
public class StreamingSinkListOptions {
    /** Limit the number of returned items */
    Long limit;

    /** The iterator returned from a prior invocation */
    String iterator;

    /** The sorting order of the returned items */
    Ordering order;
}
