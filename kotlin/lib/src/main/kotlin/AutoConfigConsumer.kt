package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.EndpointAutoConfigDeprecated
import com.svix.kotlin.internal.DestinationAutoconfig
import com.svix.kotlin.internal.MessagePollerv2
import com.svix.kotlin.internal.MessagePollerv2ConsumerCommitOptions
import com.svix.kotlin.internal.MessagePollerv2ConsumerPollOptions
import com.svix.kotlin.models.AutoConfigSinkType
import com.svix.kotlin.models.AutoConfigSinkTypeConfig
import com.svix.kotlin.models.DestinationIn
import com.svix.kotlin.models.DestinationInConfig
import com.svix.kotlin.models.DestinationOut
import com.svix.kotlin.models.DestinationOutConfig
import com.svix.kotlin.models.EndpointOut
import com.svix.kotlin.models.PollerV2CommitIn
import com.svix.kotlin.models.PollerV2PollOut
import com.svix.kotlin.models.SinkInCommon
import com.svix.kotlin.models.SinkStatus
import com.svix.kotlin.models.SubscribeIn
import okhttp3.HttpUrl.Companion.toHttpUrlOrNull

class AutoConfigConsumer
@Throws(InvalidAutoConfigTokenException::class)
constructor(token: String, sinkIn: SinkInCommon) {
    private val appId: String
    private var sinkId: String?
    private val autoconfigId: String?
    private val sinkIn: SinkInCommon
    private val httpClient: SvixHttpClient

    init {
        val content = AutoConfig.decodeAutoConfigToken(token)

        val parsedUrl =
            content.serverUrl.toHttpUrlOrNull() ?: throw InvalidAutoConfigTokenException()
        this.httpClient = SvixHttpClient(content.tokenPlaintext, parsedUrl, listOf(50, 100, 200))

        this.appId = content.appId
        this.sinkId = content.endpointId
        this.autoconfigId = content.autoconfigId
        this.sinkIn = sinkIn
    }

    /** Registers or updates the polling sink via the auto-config API. */
    @Throws(ApiException::class)
    suspend fun subscribe(): DestinationOut {
        if (autoconfigId != null) {
            val destination =
                DestinationAutoconfig(httpClient)
                    .subscribe(
                        appId,
                        autoconfigId,
                        sinkInCommonToPollingDestination(sinkIn),
                    )
            sinkId = destination.id
            return destination
        }

        val endpoint =
            EndpointAutoConfigDeprecated(httpClient)
                .update(
                    appId,
                    sinkId as String,
                    SubscribeIn(
                        sink =
                            AutoConfigSinkType(
                                config =
                                    AutoConfigSinkTypeConfig.Poller(sinkIn),
                            ),
                    ),
                )
        return destinationOutFromV1Endpoint(endpoint)
    }

    @Throws(ApiException::class)
    suspend fun receive(
        consumerId: String,
        options: MessagePollerv2ConsumerPollOptions = MessagePollerv2ConsumerPollOptions(),
    ): PollerV2PollOut {
        sinkId = sinkId ?: subscribe().id
        return MessagePollerv2(httpClient).consumerPoll(appId, sinkId as String, consumerId, options)
    }

    @Throws(ApiException::class)
    suspend fun commit(
        consumerId: String,
        offset: ULong,
        options: MessagePollerv2ConsumerCommitOptions = MessagePollerv2ConsumerCommitOptions(),
    ) {
        sinkId = sinkId ?: subscribe().id
        MessagePollerv2(httpClient)
            .consumerCommit(
                appId,
                sinkId as String,
                consumerId,
                PollerV2CommitIn(offset),
                options,
            )
    }
}

private fun destinationOutFromV1Endpoint(endpoint: EndpointOut): DestinationOut {
    return DestinationOut(
        id = endpoint.id,
        uid = endpoint.uid,
        status =
            if (endpoint.disabled == true) {
                SinkStatus.DISABLED
            } else {
                SinkStatus.ENABLED
            },
        currentIterator = "",
        createdAt = endpoint.createdAt,
        updatedAt = endpoint.updatedAt,
        batchSize = 0,
        maxWaitSecs = 0,
        eventTypes = endpoint.eventTypes?.toList(),
        channels = endpoint.channels?.toList(),
        metadata = endpoint.metadata,
        config = DestinationOutConfig.PollingEndpoint,
    )
}

private fun sinkInCommonToPollingDestination(sink: SinkInCommon): DestinationIn {
    return DestinationIn(
        uid = sink.uid,
        eventTypes = sink.eventTypes?.toList(),
        channels = sink.channels?.toList(),
        metadata = sink.metadata,
        config = DestinationInConfig.PollingEndpoint,
    )
}
