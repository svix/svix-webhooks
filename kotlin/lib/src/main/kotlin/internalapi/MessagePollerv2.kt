// this file is @generated
package com.svix.kotlin.internal

import com.svix.kotlin.SvixHttpClient
import com.svix.kotlin.serializeQueryParam
import com.svix.kotlin.models.PollerV2CommitIn
import com.svix.kotlin.models.PollerV2PollOut
import com.svix.kotlin.models.StartingPosition
import okhttp3.Headers

data class MessagePollerv2ConsumerPollOptions(
    val limit: ULong? = null,
    /** Lease duration in milliseconds. */
    val leaseDurationMs: ULong? = null,
    val startingPosition: StartingPosition? = null,
)

data class MessagePollerv2ConsumerCommitOptions(val idempotencyKey: String? = null)

class MessagePollerv2(private val client: SvixHttpClient) {
    /** Poll messages from a sink. */
    suspend fun consumerPoll(
        appId: String,
        sinkId: String,
        consumerId: String,
        options: MessagePollerv2ConsumerPollOptions = MessagePollerv2ConsumerPollOptions(),
    ): PollerV2PollOut {
        val url =
            client
                .newUrlBuilder()
                .encodedPath("/api/v1/app/$appId/polling-endpoint/$sinkId/consumer/$consumerId")
        options.limit?.let { url.addQueryParameter("limit", serializeQueryParam(it)) }
        options.leaseDurationMs?.let {
            url.addQueryParameter("lease_duration_ms", serializeQueryParam(it))
        }
        options.startingPosition?.let {
            url.addQueryParameter("starting_position", serializeQueryParam(it))
        }
        return client.executeRequest<Any, PollerV2PollOut>("GET", url.build())
    }

    /** Ack a message offset for a sink's consumer. */
    suspend fun consumerCommit(
        appId: String,
        sinkId: String,
        consumerId: String,
        pollerV2CommitIn: PollerV2CommitIn,
        options: MessagePollerv2ConsumerCommitOptions = MessagePollerv2ConsumerCommitOptions(),
    ) {
        val url =
            client
                .newUrlBuilder()
                .encodedPath(
                    "/api/v1/app/$appId/polling-endpoint/$sinkId/consumer/$consumerId/commit"
                )
        val headers = Headers.Builder()
        options.idempotencyKey?.let { headers.add("idempotency-key", it) }

        client.executeRequest<PollerV2CommitIn, Boolean>(
            "POST",
            url.build(),
            headers = headers.build(),
            reqBody = pollerV2CommitIn,
        )
    }
}
