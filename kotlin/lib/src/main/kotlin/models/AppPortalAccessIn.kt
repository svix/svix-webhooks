// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class AppPortalAccessIn(
    /**
     * Optionally creates a new application while generating the access link.
     *
     * If the application id or uid that is used in the path already exists, this argument is
     * ignored.
     */
    val application: ApplicationIn? = null,
    /**
     * Custom capabilities attached to the token, You can combine as many capabilities as necessary.
     *
     * The `ViewBase` capability is always required
     * - `ViewBase`: Basic read only permissions, does not allow the user to see the endpoint
     *   secret.
     * - `ViewEndpointSecret`: Allows user to view the endpoint secret.
     * - `ManageEndpointSecret`: Allows user to rotate and view the endpoint secret.
     * - `ManageTransformations`: Allows user to modify the endpoint transformations.
     * - `CreateAttempts`: Allows user to replay missing messages and send example messages.
     * - `ManageEndpoint`: Allows user to read/modify any field or configuration of an endpoint
     *   (including secrets)
     *
     * By default, the token will get all capabilities if the capabilities are not explicitly
     * specified.
     */
    val capabilities: Set<AppPortalCapability>? = null,
    /**
     * How long the token will be valid for, in seconds.
     *
     * Valid values are between 1 hour and 7 days. The default is 7 days.
     */
    val expiry: ULong? = null,
    /** The set of feature flags the created token will have access to. */
    val featureFlags: Set<String>? = null,
    /** Whether the app portal should be in read-only mode. */
    val readOnly: Boolean? = null,
    /**
     * An optional session ID to attach to the token.
     *
     * When expiring tokens with "Expire All", you can include the session ID to only expire tokens
     * that were created with that session ID.
     */
    val sessionId: String? = null,
)
