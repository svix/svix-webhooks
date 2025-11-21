// this file is @generated
package com.svix.api;

import com.svix.models.Ordering;

import lombok.Data;

@Data
public class ApplicationListOptions {
    /** Exclude applications that have no endpoints. Default is false. */
    Boolean excludeAppsWithNoEndpoints;

    /** Exclude applications that have only disabled endpoints. Default is false. */
    Boolean excludeAppsWithDisabledEndpoints;

    /** Limit the number of returned items */
    Long limit;

    /** The iterator returned from a prior invocation */
    String iterator;

    /** The sorting order of the returned items */
    Ordering order;
}
