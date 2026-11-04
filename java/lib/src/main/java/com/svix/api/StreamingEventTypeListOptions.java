// this file is @generated
package com.svix.api;

import com.svix.models.ArchiveStatus;
import com.svix.models.Ordering;

import lombok.Data;

@Data
public class StreamingEventTypeListOptions {
    /** Limit the number of returned items */
    Long limit;

    /** The iterator returned from a prior invocation */
    String iterator;

    /** The sorting order of the returned items */
    Ordering order;

    /**
     * Include archived items in the response.
     *
     * <p>Deprecated, use `archive_status` instead. Ignored when `archive_status` is set.
     */
    Boolean includeArchived;

    /**
     * Which items to include based on whether they are archived.
     *
     * <p>Defaults to `active`.
     */
    ArchiveStatus archiveStatus;
}
