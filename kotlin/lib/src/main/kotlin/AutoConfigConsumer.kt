package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.EndpointAutoConfig
import com.svix.kotlin.internal.MessagePollerv2
import com.svix.kotlin.internal.MessagePollerv2ConsumerCommitOptions
import com.svix.kotlin.internal.MessagePollerv2ConsumerPollOptions
import com.svix.kotlin.models.AutoConfigSinkType
import com.svix.kotlin.models.AutoConfigSinkTypeConfig
import com.svix.kotlin.models.EndpointOut
import com.svix.kotlin.models.PollerV2CommitIn
import com.svix.kotlin.models.PollerV2PollOut
import com.svix.kotlin.models.SinkInCommon
import com.svix.kotlin.models.SubscribeIn
import okhttp3.HttpUrl.Companion.toHttpUrlOrNull

class AutoConfigConsumer
@Throws(InvalidAutoConfigTokenException::class)
constructor(token: String, sinkIn: SinkInCommon) {
    private val appId: String
    private val sinkId: String
    private val sinkIn: SinkInCommon
    private val httpClient: SvixHttpClient

    init {
        val content = AutoConfig.decodeAutoConfigTokenV1(token)

        val parsedUrl =
            content.serverUrl.toHttpUrlOrNull() ?: throw InvalidAutoConfigTokenException()
        val defaultHeaders =
            mapOf(
                "User-Agent" to "svix-libs/${Version}/kotlin",
                "Authorization" to "Bearer ${content.tokenPlaintext}",
            )
        this.httpClient = SvixHttpClient(parsedUrl, defaultHeaders, listOf(50, 100, 200))

        this.appId = content.appId
        this.sinkId = content.endpointId
        this.sinkIn = sinkIn
    }

    /** Registers or updates the polling sink via the auto-config API. */
    @Throws(ApiException::class)
    suspend fun subscribe(): EndpointOut {
        val subscribeIn =
            SubscribeIn(
                sink =
                    AutoConfigSinkType(
                        config = AutoConfigSinkTypeConfig.Poller(sinkIn),
                    ),
            )
        return EndpointAutoConfig(httpClient).update(appId, sinkId, subscribeIn)
    }

    @Throws(ApiException::class)
    suspend fun receive(
        consumerId: String,
        options: MessagePollerv2ConsumerPollOptions = MessagePollerv2ConsumerPollOptions(),
    ): PollerV2PollOut {
        return MessagePollerv2(httpClient).consumerPoll(appId, sinkId, consumerId, options)
    }

    @Throws(ApiException::class)
    suspend fun commit(
        consumerId: String,
        offset: ULong,
        options: MessagePollerv2ConsumerCommitOptions = MessagePollerv2ConsumerCommitOptions(),
    ) {
        MessagePollerv2(httpClient)
            .consumerCommit(
                appId,
                sinkId,
                consumerId,
                PollerV2CommitIn(offset),
                options,
            )
    }
}
