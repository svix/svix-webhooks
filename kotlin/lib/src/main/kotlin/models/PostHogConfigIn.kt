// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class PostHogConfigIn(
    /**
     * The PostHog ingestion domain.
     *
     * This is https://us.i.posthog.com for US Cloud, https://eu.i.posthog.com for EU Cloud, and
     * your self-hosted domain for self-hosted instances. Note the `.i.` in the Cloud hosts
     * (`us.posthog.com` and `eu.posthog.com` are different, private endpoints).
     */
    val domain: String,
    /**
     * The PostHog project API key.
     *
     * Sent as the `api_key` field in the body of each `/batch/` request.
     */
    val projectToken: String,
)
