// this file is @generated
package com.svix.api;

import lombok.Data;

@Data
public class EventTypeDeleteOptions {
    /**
     * By default event types are archived when "deleted". Passing this to `true` deletes them
     * entirely.
     */
    Boolean expunge;
}
