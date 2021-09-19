package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.MessageAttemptApi
import com.svix.kotlin.internal.infrastructure.ClientException
import com.svix.kotlin.internal.infrastructure.ServerException
import com.svix.kotlin.models.ListResponseEndpointMessageOut
import com.svix.kotlin.models.ListResponseMessageAttemptEndpointOut
import com.svix.kotlin.models.ListResponseMessageAttemptOut
import com.svix.kotlin.models.ListResponseMessageEndpointOut
import com.svix.kotlin.models.MessageAttemptOut
import com.svix.kotlin.models.MessageStatus

class MessageAttemptListOptions(var messageStatus: MessageStatus? = null) : ListOptions() {
    fun messageStatus(messageStatus: MessageStatus) = apply { this.messageStatus = messageStatus }

    override fun iterator(iterator: kotlin.String) = apply { super.iterator(iterator) }

    override fun limit(limit: kotlin.Int) = apply { super.limit(limit) }
}

class MessageAttempt internal constructor(debugUrl: String) {
    val api = MessageAttemptApi(debugUrl)

    suspend fun list(appId: String, msgId: String, options: MessageAttemptListOptions): ListResponseMessageAttemptOut {
        try {
            return api.listAttemptsApiV1AppAppIdMsgMsgIdAttemptGet(
                appId,
                msgId,
                options.iterator,
                options.limit,
                options.messageStatus
            )
        } catch (e: Exception) {
            when (e) {
                is ServerException, is ClientException, is UnsupportedOperationException -> {
                    throw ApiException(e)
                }
                else -> throw e
            }
        }
    }

    suspend fun get(msgId: String, appId: String, attemptId: String): MessageAttemptOut {
        try {
            return api.getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(attemptId, msgId, appId)
        } catch (e: Exception) {
            when (e) {
                is ServerException, is ClientException, is UnsupportedOperationException -> {
                    throw ApiException(e)
                }
                else -> throw e
            }
        }
    }

    suspend fun resend(msgId: String, appId: String, endpointId: String) {
        try {
            api.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(endpointId, msgId, appId)
        } catch (e: Exception) {
            when (e) {
                is ServerException, is ClientException, is UnsupportedOperationException -> {
                    throw ApiException(e)
                }
                else -> throw e
            }
        }
    }

    suspend fun listAttemptedMessages(
        appId: String,
        endpointId: String,
        options: MessageAttemptListOptions
    ): ListResponseEndpointMessageOut {
        try {
            return api.listAttemptedMessagesApiV1AppAppIdEndpointEndpointIdMsgGet(
                appId,
                endpointId,
                options.iterator,
                options.limit,
                options.messageStatus
            )
        } catch (e: Exception) {
            when (e) {
                is ServerException, is ClientException, is UnsupportedOperationException -> {
                    throw ApiException(e)
                }
                else -> throw e
            }
        }
    }

    suspend fun listAttemptedDestinations(
        msgId: String,
        appId: String,
        options: MessageAttemptListOptions
    ): ListResponseMessageEndpointOut {
        try {
            return api.listAttemptedDestinationsApiV1AppAppIdMsgMsgIdEndpointGet(
                msgId,
                appId,
                options.iterator,
                options.limit
            )
        } catch (e: Exception) {
            when (e) {
                is ServerException, is ClientException, is UnsupportedOperationException -> {
                    throw ApiException(e)
                }
                else -> throw e
            }
        }
    }

    suspend fun listAttemptsForEndpoint(
        msgId: String,
        appId: String,
        endpointId: String,
        options: MessageAttemptListOptions
    ): ListResponseMessageAttemptEndpointOut {
        try {
            return api.listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(
                msgId,
                appId,
                endpointId,
                options.iterator,
                options.limit,
                options.messageStatus
            )
        } catch (e: Exception) {
            when (e) {
                is ServerException, is ClientException, is UnsupportedOperationException -> {
                    throw ApiException(e)
                }
                else -> throw e
            }
        }
    }
}
