// this file is @generated
package com.svix.api;

import lombok.Data;

@Data
public class MessageCreateOptions {
    /** When `true`, message payloads are included in the response. */
    Boolean withContent;

    String idempotencyKey;
}
