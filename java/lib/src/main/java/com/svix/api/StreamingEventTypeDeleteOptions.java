// this file is @generated
package com.svix.api;

import lombok.Data;

@Data
public class StreamingEventTypeDeleteOptions {
    /**
     * By default, event types are archived when "deleted". With this flag, they are deleted
     * entirely.
     */
    Boolean expunge;
}
