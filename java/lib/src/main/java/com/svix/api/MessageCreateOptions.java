// this file is @generated
package com.svix.api;

import lombok.Data;

@Data
public class MessageCreateOptions {
    /**
     * When `true`, message payloads are included in the response.
     *
     * <p>Defaults to `false` in v2+ of the Svix SDKs, `true` in v1 or when manually making a
     * request without specifying this parameter.
     */
    Boolean withContent;

    String idempotencyKey;
}
