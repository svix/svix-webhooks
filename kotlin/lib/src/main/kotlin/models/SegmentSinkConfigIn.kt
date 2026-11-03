// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class SegmentSinkConfigIn(
    /**
     * The Segment ingestion host.
     *
     * This is https://api.segment.io for US workspaces, https://events.eu1.segmentapis.com for EU
     * workspaces, or your custom domain. Events are sent to the `/v1/batch` endpoint under this
     * host.
     */
    val host: String,
    /**
     * The Segment source write key.
     *
     * Sent as HTTP Basic auth (the write key as the username, with an empty password) on each
     * `/v1/batch` request.
     */
    val writeKey: String,
)
