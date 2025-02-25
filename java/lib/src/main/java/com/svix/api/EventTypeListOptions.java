// this file is @generated
package com.svix.api;

import com.svix.models.Ordering;

import lombok.Data;

@Data
public class EventTypeListOptions {
    /** Limit the number of returned items */
    Long limit;

    /** The iterator returned from a prior invocation */
    String iterator;

    /** The sorting order of the returned items */
    Ordering order;

    /** When `true` archived (deleted but not expunged) items are included in the response. */
    Boolean includeArchived;

    /** When `true` the full item (including the schema) is included in the response. */
    Boolean withContent;
}
