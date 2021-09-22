package com.svix.kotlin

import com.svix.kotlin.exceptions.ApiException
import com.svix.kotlin.internal.apis.MessageAttemptApi
import com.svix.kotlin.models.ListResponseEndpointMessageOut
import com.svix.kotlin.models.ListResponseMessageAttemptEndpointOut
import com.svix.kotlin.models.ListResponseMessageAttemptOut
import com.svix.kotlin.models.ListResponseMessageEndpointOut
import com.svix.kotlin.models.MessageAttemptOut

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
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun get(msgId: String, appId: String, attemptId: String): MessageAttemptOut {
        try {
            return api.getAttemptApiV1AppAppIdMsgMsgIdAttemptAttemptIdGet(attemptId, msgId, appId)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun resend(msgId: String, appId: String, endpointId: String) {
        try {
            api.resendWebhookApiV1AppAppIdMsgMsgIdEndpointEndpointIdResendPost(endpointId, msgId, appId)
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
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
                options.messageStatus,
                options.before
            )
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
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
            throw ApiException.wrapInternalApiException(e)
        }
    }

    suspend fun listAttemptsForEndpoint(
        msgId: String,
        appId: String,
        endpointId: String,
        options: MessageAttemptListOptions
    ): ListResponseMessageAttemptEndpointOut {
        return try {
            api.listAttemptsForEndpointApiV1AppAppIdMsgMsgIdEndpointEndpointIdAttemptGet(
                msgId,
                appId,
                endpointId,
                options.iterator,
                options.limit,
                options.eventTypes,
                options.messageStatus,
                options.before
            )
        } catch (e: Exception) {
            throw ApiException.wrapInternalApiException(e)
        }
    }
}
