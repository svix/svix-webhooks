// this file is @generated
package com.svix.api;

import com.svix.models.BackgroundTaskStatus;
import com.svix.models.BackgroundTaskType;
import com.svix.models.Ordering;

import lombok.Data;

@Data
public class BackgroundTaskListOptions {
    /** Filter the response based on the status. */
    BackgroundTaskStatus status;

    /** Filter the response based on the type. */
    BackgroundTaskType task;

    /** Limit the number of returned items */
    Long limit;

    /** The iterator returned from a prior invocation */
    String iterator;

    /** The sorting order of the returned items */
    Ordering order;
}
